use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let ghc_dir = std::env::var("GHC_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.parent().unwrap().parent().unwrap().to_owned());

    let object_file = ghc_dir
        .join("_build")
        .join("stage1")
        .join("compiler")
        .join("build")
        .join("GHC")
        .join("StgToJS")
        .join("Rts")
        .join("Rts.o");

    let mut include_dir = ghc_dir.join("_build");
    include_dir.push("stage1");
    include_dir.push("lib");
    include_dir.push("TODO-platform-specific");
    include_dir.push("rts-1.0.2");
    include_dir.push("include");
    let include_dir = include_dir;

    let header_file = include_dir.join("Rts.h");

    println!("cargo::rustc-link-lib=static={}", object_file.display());

    let bindings = bindgen::Builder::default()
        .header(header_file.to_str().unwrap())
        .clang_arg(format!("-I{}", include_dir.display()))
        // Invalidate bindings when header files change.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("rts.rs"))
        .expect("Failed writing bindings");
}
