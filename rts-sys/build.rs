use build_utils::{self as utils, Ways};
use std::io::Write as _;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // TODO: Use features for ways.
    let ghc = utils::GhcConfig::new(Ways {
        threaded: true,
        debug: true,
        profiling: true,
        dynamic: true,
    });

    utils::rustc_link(&ghc);

    let bindings = ghc.rts_bindings(true);

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
