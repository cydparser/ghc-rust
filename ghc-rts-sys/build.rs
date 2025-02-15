use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let ghc_dir = std::env::var("GHC_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.parent().unwrap().parent().unwrap().to_owned());

    let object_file = {
        let mut path = ghc_dir.join("_build");
        path.extend([
            "stage1", "compiler", "build", "GHC", "StgToJS", "Rts", "Rts.o",
        ]);
        path
    };

    let include_dir = {
        let mut include_dir = ghc_dir.clone();
        include_dir.extend(["_build", "stage1", "lib"]);

        let arch_os = format!("{}-{}-ghc-", std::env::consts::ARCH, std::env::consts::OS);

        let arch_os_file_name = include_dir
            .read_dir()
            .unwrap()
            .find_map(|entry| {
                let entry = entry.unwrap();
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                if file_name_str.starts_with(&arch_os) {
                    Some(String::from(file_name_str))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("unable to locate directory prefixed with {}", arch_os));

        include_dir.extend([&arch_os_file_name, "rts-1.0.2", "include"]);
        include_dir
    };

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
