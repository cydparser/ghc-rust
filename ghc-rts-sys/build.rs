use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let ghc_dir = std::env::var("GHC_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.parent().unwrap().parent().unwrap().to_owned());

    let build_dir = {
        let mut path = ghc_dir.clone();
        path.extend(["_build", "stage1", "rts", "build"]);
        path
    };

    println!("cargo::rustc-link-search={}", build_dir.display());

    println!("cargo::rustc-link-lib=HSrts-1.0.2");

    let include_dir = {
        let mut path = ghc_dir.clone();
        path.extend(["rts", "include"]);
        path
    };

    let bindings = bindgen::Builder::default()
        .header(include_dir.join("Rts.h").to_string_lossy())
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg(format!("-I{}", build_dir.join("include").display()))
        // Invalidate bindings when header files change.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_file(format!("{}.*", include_dir.as_os_str().to_string_lossy()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("rts.rs"))
        .expect("Failed writing bindings");
}
