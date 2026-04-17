use std::{
    env,
    path::{Path, PathBuf},
};

use cbindgen::Language;

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let project_dir = manifest_dir.parent().unwrap().parent().unwrap();

    let args = std::env::args_os().skip(1);

    let dst_dir = match args.collect::<Vec<_>>().as_slice() {
        [path] => PathBuf::from(path),
        _ => project_dir.join("include"),
    };

    let builder = cbindgen::Builder::new()
        .with_include_version(true)
        .with_pragma_once(true)
        .with_language(Language::C)
        .with_cpp_compat(true)
        .with_define("target_pointer_width", "64", "SIZEOF_VOID_P_8")
        .with_define("target_pointer_width", "32", "SIZEOF_VOID_P_4")
        .with_parse_expand_default_features(false)
        .with_parse_expand_features(&["header"]);

    let generate = |header, builder: cbindgen::Builder| {
        builder
            .generate()
            .expect(&format!("generate header {header}"))
            .write_to_file(dst_dir.join(header));
    };

    generate(
        "Rts.h",
        builder
            .clone()
            .with_crate(project_dir.join("rts"))
            .with_include("target.h")
            .with_after_include(
                "\n\
             typedef struct StgTSO_ StgTSO;\n\
             typedef struct bdescr_ bdescr;",
            ),
    );
    generate(
        "rts/Signals.h",
        builder.with_src(project_dir.join("rts/src/ffi/rts/signals.rs")),
    );
}
