use std::fs;
#[cfg(unix)]
use std::os::unix;
use std::path::{Path, PathBuf};

pub struct GhcDirs {
    pub root_dir: PathBuf,
    pub rts_dir: PathBuf,
    pub include_dir: PathBuf,
    pub build_dir: PathBuf,
}

impl GhcDirs {
    pub fn new() -> GhcDirs {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

        let ghc_dir = std::env::var("GHC_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| manifest_dir.parent().unwrap().parent().unwrap().to_owned());

        GhcDirs {
            root_dir: ghc_dir.clone(),
            rts_dir: ghc_dir.join(PathBuf::from("rts")),
            include_dir: ghc_dir.join(PathBuf::from("rts/include")),
            build_dir: ghc_dir.join(PathBuf::from("_build/stage1/rts/build")),
        }
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
        while let Some(ty) = types.next() {
            s.push_str("|");
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
        .clang_arg(format!("-I{}", ghc.build_dir.join("include").display()))
        .blocklist_type(block_types)
}

pub fn use_libc() -> String {
    let mut s = String::from("use libc::{");

    for ty in LIBC_TYPES {
        s.push_str(ty);
        s.push_str(",");
    }
    s.push_str("};\n");
    s
}

/// Configure linking. NB: This has only been tested on Linux.
pub fn rustc_link_dyn(ghc: &GhcDirs, create_symlinks: bool) {
    let outputs_lib_dir = if create_symlinks {
        let project_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

        // The executable's RUNPATH includes outputs/out/lib (on Linux, at least).
        let outputs_lib_dir = project_dir.join("outputs/out/lib");

        if outputs_lib_dir.exists() {
            fs::remove_dir_all(&outputs_lib_dir).unwrap();
        }
        fs::create_dir_all(&outputs_lib_dir).unwrap();
        Some(outputs_lib_dir)
    } else {
        None
    };

    let lib_dir = {
        let mut lib_dir = ghc.root_dir.join("_build/stage1/lib/");

        let file_names = file_names_starting_with(
            &lib_dir,
            &format!("{}-{}-ghc-", std::env::consts::ARCH, std::env::consts::OS),
        );
        assert!(file_names.len() == 1);
        lib_dir.push(PathBuf::from(&file_names[0]));
        lib_dir
    };

    println!("cargo::rustc-link-search=native={}", lib_dir.display());

    let file_names = file_names_starting_with(&lib_dir, "libHS");

    let mut libs = Libs::default();

    for file_name in file_names {
        if file_name.starts_with("libHSrts-1.0.2_thr-") {
            assert!(libs.rts.is_none());
            libs.rts = Some(file_name);
        } else if file_name.contains("-inplace-ghc") {
            if file_name.starts_with("libHSghc-bignum-") {
                assert!(libs.ghc_bignum.is_none());
                libs.ghc_bignum = Some(file_name);
            } else if file_name.starts_with("libHSghc-internal-") {
                assert!(libs.ghc_internal.is_none());
                libs.ghc_internal = Some(file_name);
            } else if file_name.starts_with("libHSghc-prim-") {
                assert!(libs.ghc_prim.is_none());
                libs.ghc_prim = Some(file_name);
            }
        }
    }

    for lib in [
        libs.rts.unwrap(),
        libs.ghc_bignum.unwrap(),
        libs.ghc_internal.unwrap(),
        libs.ghc_prim.unwrap(),
    ] {
        #[cfg(unix)]
        if let Some(ref outputs_lib_dir) = outputs_lib_dir {
            unix::fs::symlink(lib_dir.join(&lib), outputs_lib_dir.join(&lib)).unwrap();
        }
        println!("cargo::rustc-link-lib=dylib:+verbatim={}", lib);
    }
}

#[derive(Default)]
struct Libs {
    rts: Option<String>,
    ghc_bignum: Option<String>,
    ghc_internal: Option<String>,
    ghc_prim: Option<String>,
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
    return file_names;
}
