#![allow(clippy::new_without_default)]

use std::fs;
#[cfg(target_os = "linux")]
use std::os::unix;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct GhcDirs {
    pub root_dir: PathBuf,
    pub lib_dir: PathBuf,
    pub rts_dir: PathBuf,
    pub include_dir: PathBuf,
}

const RTS_VER: &str = "1.0.3";

const GHC_LIB_DIR: &str = "GHC_LIB_DIR";

/// The following environmental overrides are available:
///   - GHC_DIR: Change the path to GHC source
///   - GHC_LIB_DIR: Change the path to object files and headers
///     E.g. GHC_LIB_DIR="/nix/store/hash-ghc-ver/lib/ghc-ver/lib"
impl GhcDirs {
    pub fn new() -> GhcDirs {
        let ghc_dir = std::env::var("GHC_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
                manifest_dir.parent().unwrap().parent().unwrap().to_owned()
            });

        let lib_dir = {
            let mut lib_dir = option_env!("GHC_LIB_DIR")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let path = PathBuf::from(s);
                    if !path.exists() {
                        panic!("Invalid {GHC_LIB_DIR} path: {}", path.display());
                    }
                    path
                })
                .unwrap_or(ghc_dir.join(PathBuf::from("_build/stage1/lib")));

            let os = match std::env::consts::OS {
                "macos" => "osx",
                os => os,
            };

            let arch_os_dir = file_names_starting_with(
                &lib_dir,
                format!("{}-{}-ghc-", std::env::consts::ARCH, os),
            );
            lib_dir.push(PathBuf::from(&arch_os_dir[0]));
            lib_dir
        };

        let rts_dir = lib_dir.join(PathBuf::from(format!("rts-{}", RTS_VER)));
        let include_dir = rts_dir.join(PathBuf::from("include"));

        GhcDirs {
            root_dir: ghc_dir.clone(),
            lib_dir,
            rts_dir,
            include_dir,
        }
    }

    pub fn rts_bindings(&self) -> bindgen::Bindings {
        bindgen_builder(self)
            .header(self.include_dir.join("Rts.h").to_string_lossy())
            // Invalidate bindings when header files change.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .allowlist_file(format!(
                "{}.*",
                self.include_dir.as_os_str().to_string_lossy()
            ))
            .generate()
            .expect("unable to generate bindings")
    }
}

static LIBC_TYPES: [&str; 6] = [
    "clockid_t",
    "pid_t",
    "pthread_t",
    "pthread_cond_t",
    "pthread_key_t",
    "pthread_mutex_t",
];

pub fn bindgen_builder(ghc: &GhcDirs) -> bindgen::Builder {
    let block_types: String = {
        let mut s = String::from("\\b(");
        let mut types = LIBC_TYPES.iter();
        s.push_str(types.next().unwrap());

        for ty in types {
            s.push('|');
            s.push_str(ty);
        }
        s.push_str(")\\b");
        s
    };
    bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
        .use_core()
        .clang_arg(format!("-I{}", ghc.include_dir.display()))
        .blocklist_type(block_types)
}

pub fn use_libc() -> String {
    let mut s = String::from("use libc::{");

    let libc_types: Vec<&str> = if std::env::consts::OS == "macos" {
        LIBC_TYPES
            .into_iter()
            .filter(|s| *s != "clockid_t")
            .collect()
    } else {
        LIBC_TYPES.into()
    };

    for ty in libc_types {
        s.push_str(ty);
        s.push(',');
    }
    s.push_str("};\n");
    s
}

/// Configure linking.
///
/// When `create_symlinks` is true and `cfg!(target_os = "linux")`, _libHS_ shared objects will be
/// symlinked into _outputs/out/lib_ in order to avoid needing to set LD_LIBRARY_PATH for
/// tests/executables.
pub fn rustc_link(ghc: &GhcDirs, create_symlinks: bool) {
    dbg!(ghc);
    println!("cargo::rustc-link-arg=--verbose");

    // Disable PIE for tests. `cargo::rustc-link-arg-tests` does not work for unit tests
    // (https://github.com/rust-lang/cargo/issues/10937).
    #[cfg(target_os = "linux")]
    println!("cargo::rustc-link-arg=-Wl,--no-pie");

    #[cfg(target_os = "macos")]
    println!("cargo::rustc-link-arg=-Wl,-rpath,{}", ghc.lib_dir.display());

    println!("cargo::rustc-link-search=native={}", ghc.lib_dir.display());

    #[allow(unused_variables)]
    let outputs_lib_dir = if create_symlinks && cfg!(target_os = "linux") {
        let project_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

        // The test executable's RUNPATH includes outputs/out/lib (on Linux, at least).
        let outputs_lib_dir = project_dir.join("outputs/out/lib");

        if outputs_lib_dir.exists() {
            fs::remove_dir_all(&outputs_lib_dir).unwrap();
        }
        fs::create_dir_all(&outputs_lib_dir).unwrap();
        Some(outputs_lib_dir)
    } else {
        None
    };

    let lib_rts = format!("libHSrts-{}_thr-ghc", RTS_VER);

    let mut libs = [
        (lib_rts.as_ref(), None),
        ("libHSghc-bignum-", None),
        ("libHSghc-internal-", None),
        ("libHSghc-prim-", None),
    ];

    let lib_predicate: fn(&String) -> bool = if std::env::var(GHC_LIB_DIR).is_ok() {
        |s| !s.contains("_p-ghc")
    } else {
        |s| s.contains("-inplace-ghc") || s.starts_with("libHSrts")
    };

    for file_name in file_names_starting_with(&ghc.lib_dir, "libHS")
        .into_iter()
        .filter(lib_predicate)
    {
        if let Some((prefix, lib)) = libs
            .iter_mut()
            .find(|&&mut (prefix, _)| file_name.starts_with(prefix))
        {
            assert!(lib.is_none(), "Multiple libs start with {}", prefix);
            *lib = Some(file_name);
        };
    }

    for (prefix, lib) in libs {
        let lib = lib.expect(prefix);

        // The linker on Linux doesn't accept the library names produced by GHC, so we use
        // `+verbatim`. On macOS, ld64 doesn't support verbatim.
        if cfg!(target_os = "macos") {
            let lib = lib[3..].strip_suffix(".dylib").unwrap();
            println!("cargo::rustc-link-lib=dylib={}", lib);
        } else {
            println!("cargo::rustc-link-lib=dylib:+verbatim={}", lib);
        }

        #[cfg(target_os = "linux")]
        if let Some(ref outputs_lib_dir) = outputs_lib_dir {
            unix::fs::symlink(ghc.lib_dir.join(&lib), outputs_lib_dir.join(&lib)).unwrap();
        }
    }

    for lib in [
        #[cfg(target_os = "linux")]
        "numa",
        #[cfg(target_os = "linux")]
        "dw",
        #[cfg(target_os = "linux")]
        "elf",
        "ffi",
        "gmp",
    ] {
        println!("cargo::rustc-link-lib={}", lib);
    }
}

fn file_names_starting_with<P: AsRef<Path>, Pat: AsRef<str>>(dir: P, pat: Pat) -> Vec<String> {
    let mut file_names = vec![];

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();

        if file_name.starts_with(pat.as_ref()) {
            file_names.push(file_name.to_string());
        }
    }
    file_names
}
