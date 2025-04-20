use std::io::Write as _;
#[cfg(unix)]
use std::os::unix;
use std::path::{Path, PathBuf};
use std::{env, fs};

use bindgen_utils::{self as utils, GhcDirs};

fn main() {
    let ghc = utils::GhcDirs::new();

    if cfg!(unix) {
        rustc_link(&ghc);
    } else {
        panic!("Windows is not yet supported");
    }

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

/// Configure linker. NB: This has only been tested on Linux.
fn rustc_link(ghc: &GhcDirs) {
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

    // The executable's RUNPATH includes outputs/out/lib (on Linux, at least).
    let outputs_lib_dir = project_dir.join("outputs/out/lib");

    if outputs_lib_dir.exists() {
        fs::remove_dir_all(&outputs_lib_dir).unwrap();
    }
    fs::create_dir_all(&outputs_lib_dir).unwrap();

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

    println!(
        "cargo::rustc-link-search=native={}",
        ghc.build_dir.display()
    );
    println!("cargo::rustc-link-search=native={}", lib_dir.display());

    let file_names = file_names_starting_with(&lib_dir, "libHS");

    struct Libs {
        rts: Option<String>,
        ghc_bignum: Option<String>,
        ghc_internal: Option<String>,
        ghc_prim: Option<String>,
    }
    let mut libs = Libs {
        rts: None,
        ghc_bignum: None,
        ghc_internal: None,
        ghc_prim: None,
    };

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
        unix::fs::symlink(lib_dir.join(&lib), outputs_lib_dir.join(&lib)).unwrap();
        println!("cargo::rustc-link-lib=dylib:+verbatim={}", lib);
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
    return file_names;
}
