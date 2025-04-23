use std::io::Write as _;
use std::path::PathBuf;
use std::{env, fs};

use build_utils as utils;

fn main() {
    let ghc = utils::GhcDirs::new();

    utils::rustc_link_dyn(&ghc, cfg!(unix));

    let bindings = utils::bindgen_builder(&ghc)
        .header(ghc.include_dir.join("Rts.h").to_string_lossy())
        // Invalidate bindings when header files change.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_file(format!(
            "{}.*",
            ghc.include_dir.as_os_str().to_string_lossy()
        ))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut out_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(out_path.join("rts.rs"))
        .unwrap();

    out_file.write_all(utils::use_libc().as_bytes()).unwrap();

    bindings
        .write(Box::new(out_file))
        .expect("Failed writing bindings");
}
