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

            let arch_os_prefix = format!("{}-{}-ghc-", std::env::consts::ARCH, os);

            let arch_os_dir = find_paths(&lib_dir, &|s| s.starts_with(&arch_os_prefix), 0);
            lib_dir.push(PathBuf::from(&arch_os_dir[0].1));
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

    pub fn rts_bindings(&self, build_rs: bool) -> bindgen::Bindings {
        let mut builder =
            bindgen_builder(self).header(self.include_dir.join("Rts.h").to_string_lossy());

        if cfg!(windows) {
            builder = builder.allowlist_file(".*\\bstage1\\b.*");
        } else {
            builder = builder.allowlist_file(format!("{}.*", self.include_dir.to_string_lossy()));
        }

        if build_rs {
            // Invalidate bindings when header files change.
            builder = builder.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
        }

        builder.generate().expect("unable to generate bindings")
    }
}

static LIBC_TYPES: [&str; 5] = [
    "clockid_t",
    "pid_t",
    "pthread_t",
    "pthread_cond_t",
    "pthread_mutex_t",
];

pub fn bindgen_builder(ghc: &GhcDirs) -> bindgen::Builder {
    let mut builder = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
        .generate_cstr(true)
        .use_core()
        .clang_arg(format!("-I{}", ghc.include_dir.display()));

    if cfg!(windows) {
        builder = builder.clang_arg(format!(
            "-I{}",
            ghc.root_dir.join("inplace/mingw/include").display()
        ));
    } else {
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
        builder = builder.blocklist_type(block_types);
    }

    builder
}

pub fn use_libc() -> String {
    if cfg!(windows) {
        return String::new();
    }

    let mut s = String::from("use libc::{");

    let libc_types: Vec<&str> = if cfg!(target_os = "macos") {
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

    let lib_rts = if cfg!(windows) {
        format!("libHSrts-{}_thr.a", RTS_VER)
    } else {
        format!("libHSrts-{}_thr-ghc", RTS_VER)
    };

    let mut libs = [
        (lib_rts.as_ref(), None),
        ("libHSghc-bignum-", None),
        ("libHSghc-internal-", None),
        ("libHSghc-prim-", None),
    ];

    let lib_predicate: fn(&str) -> bool = if std::env::var(GHC_LIB_DIR).is_ok() {
        |s| (s.starts_with("libHSrts") || s.starts_with("libHSghc")) && !s.contains("_p-ghc")
    } else {
        |s| {
            s.starts_with("libHSrts")
                || (s.starts_with("libHSghc")
                    && if cfg!(windows) {
                        s.ends_with("inplace.a")
                    } else {
                        s.contains("-inplace-ghc")
                    })
        }
    };

    for (path, file_name) in find_paths(
        &ghc.lib_dir,
        &lib_predicate,
        if cfg!(windows) { 1 } else { 0 },
    ) {
        if let Some((prefix, lib)) = libs
            .iter_mut()
            .find(|&&mut (prefix, _)| file_name.starts_with(prefix))
        {
            assert!(lib.is_none(), "Multiple libs start with {}", prefix);
            println!(
                "cargo::rustc-link-search={}",
                path.parent().unwrap().display()
            );
            *lib = Some((path, file_name));
        };
    }

    for (prefix, lib) in libs {
        #[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
        let (path, lib) = lib.expect(prefix);

        // The linker on Linux doesn't accept the library names produced by GHC, so we use
        // `+verbatim`. On macOS, ld64 doesn't support verbatim.
        if cfg!(target_os = "macos") {
            println!(
                "cargo::rustc-link-lib=dylib={}",
                lib[3..].strip_suffix(".dylib").unwrap()
            );
        } else if cfg!(windows) {
            println!(
                "cargo::rustc-link-lib=static={}",
                lib[3..].strip_suffix(".a").unwrap()
            );
        } else {
            println!("cargo::rustc-link-lib=dylib:+verbatim={}", lib);
        }

        #[cfg(target_os = "linux")]
        if let Some(ref outputs_lib_dir) = outputs_lib_dir {
            unix::fs::symlink(path, outputs_lib_dir.join(&lib)).unwrap();
        }
    }

    #[cfg(windows)]
    println!(
        "cargo::rustc-link-search={}",
        ghc.root_dir.join("_build/stage1/gmp").display()
    );

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

    #[cfg(windows)]
    for lib in ["ole32", "rpcrt4", "shell32", "ucrt"] {
        println!("cargo::rustc-link-lib={}", lib);
    }
}

fn find_paths<P>(dir: &Path, predicate: &P, max_depth: usize) -> Vec<(PathBuf, String)>
where
    P: Fn(&str) -> bool,
{
    let mut paths = vec![];

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();

        if predicate(file_name.as_ref()) {
            let file_name = file_name.to_string();
            paths.push((path, file_name));
        } else if entry.file_type().unwrap().is_dir() && max_depth > 0 {
            paths.extend(find_paths(path.as_ref(), predicate, max_depth - 1));
        }
    }

    paths
}
