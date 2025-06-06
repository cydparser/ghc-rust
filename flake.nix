{
  description = "ghc-rts";

  inputs = {
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };

    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    systems.url = "github:nix-systems/default";
  };

  nixConfig = {
    extra-substituters = [ "https://nix-community.cachix.org" ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  outputs =
    inputs@{
      self,
      flake-utils,
      nixpkgs,
      ...
    }:
    let
      systems = if builtins ? currentSystem then [ builtins.currentSystem ] else import inputs.systems;
    in
    flake-utils.lib.eachSystem systems (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ inputs.fenix.overlays.default ];
        };

        inherit (pkgs) lib;

        rustToolchain = pkgs.fenix.stable;

        crane = (inputs.crane.mkLib pkgs).overrideToolchain (
          rustToolchain.withComponents [
            "cargo"
            "clippy"
            "llvm-tools"
            "rust-analyzer-preview"
            "rust-src"
            "rustc"
            "rustfmt"
          ]
        );

        src = crane.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = [ ] ++ lib.optionals pkgs.stdenv.isDarwin [ ];
        };

        cargoArtifacts = crane.buildDepsOnly (commonArgs // { pname = "ghc-rts-"; });

        individualCrateArgs = commonArgs // {
          inherit cargoArtifacts;
          inherit (crane.crateNameFromCargoToml { inherit src; }) version;
          doCheck = false;
        };

        filesetForCrate =
          path:
          lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              (crane.fileset.commonCargoSources path)
            ];
          };

        craneBuildPackage =
          name: args:
          crane.buildPackage (
            individualCrateArgs
            // {
              pname = name;
              cargoExtraArgs = "-p ${name}";
              src = filesetForCrate ./${name};
            }
            // args
          );

        ghc = pkgs.haskell.compiler.ghc912;
      in
      {
        checks = {
          clippy = crane.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          doc = crane.cargoDoc (commonArgs // { inherit cargoArtifacts; });

          audit = crane.cargoAudit {
            inherit src;
            inherit (inputs) advisory-db;
          };

          deny = crane.cargoDeny { inherit src; };

          nextest = crane.cargoNextest (
            commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
              cargoNextestPartitionsExtraArgs = "--no-tests=pass";
            }
          );

          pre-commit = inputs.git-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              treefmt = {
                enable = true;
                settings.formatters = [
                  crane.rustfmt
                  pkgs.nixfmt-rfc-style
                  pkgs.taplo
                ];
              };
            };
          };
        };

        packages = {
          build-utils = craneBuildPackage "build-utils" { };
        };

        devShells.default = crane.devShell {
          checks = self.checks.${system};

          inputsFrom = lib.attrsets.attrValues self.packages.${system};

          packages =
            with pkgs;
            [
              llvmPackages.clang
              # Libraries
              gmp.dev
              libffi.dev
            ]
            ++ lib.optionals pkgs.stdenv.isLinux [
              elfutils.dev
              numactl.dev
            ]
            ++ self.checks.${system}.pre-commit.enabledPackages;

          shellHook =
            ''
              FLAKE_ROOT="$(git rev-parse --show-toplevel)"

              export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
              export TAPLO_CONFIG=" $FLAKE_ROOT/taplo.toml";

              export GHC_LIB_DIR=${lib.optionalString pkgs.stdenv.isDarwin "${ghc}/lib/ghc-${ghc.version}/lib"}
            ''
            + self.checks.${system}.pre-commit.shellHook;
        };
      }
    );
}
