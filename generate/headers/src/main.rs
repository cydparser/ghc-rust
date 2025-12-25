use std::{env, path::Path};

use cbindgen::Language;

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let project_dir = manifest_dir.parent().unwrap().parent().unwrap();

    cbindgen::Builder::new()
        .with_pragma_once(true)
        .with_language(Language::C)
        .with_include("target.h")
        .with_cpp_compat(true)
        .with_define("target_pointer_width", "64", "SIZEOF_VOID_P_8")
        .with_define("target_pointer_width", "32", "SIZEOF_VOID_P_4")
        .with_after_include(
            "\n\
             typedef struct StgTSO_ StgTSO;\n\
             typedef struct bdescr_ bdescr;",
        )
        .with_crate(project_dir.join("rts"))
        .with_parse_expand_default_features(false)
        .generate()
        .expect("Unable to generate header")
        .write_to_file("include/Rts.h");
}
