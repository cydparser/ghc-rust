use std::io::Write as _;
use std::path::PathBuf;
use std::{env, fs};

use build_utils as utils;

fn main() {
    let ghc = utils::GhcDirs::new();

    utils::rustc_link(&ghc, cfg!(unix));

    let bindings = ghc.rts_bindings();

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
