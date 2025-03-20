use std::path::{Path, PathBuf};

pub struct GhcDirs {
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
        .clang_arg(format!("-I{}", ghc.include_dir.display()))
        .clang_arg(format!("-I{}", ghc.build_dir.join("include").display()))
        // Invalidate bindings when header files change.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_type(block_types)
}

pub fn use_libc() -> String {
    let mut s = String::from("use libc::{");

    for ty in LIBC_TYPES {
        s.push_str(ty);
        s.push_str(",");
    }
    s.push_str("}");
    s
}
