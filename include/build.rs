use cbindgen;

use std::{env, path::Path};

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let config = cbindgen::Config::from_file(manifest_dir.join("cbindgen.toml")).unwrap();

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(manifest_dir.parent().unwrap().join("rts"))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("Rts.h");
}
