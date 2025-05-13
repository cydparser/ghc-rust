use build_utils as utils;

fn main() {
    if cfg!(feature = "sys") {
        let ghc = utils::GhcDirs::new();

        utils::rustc_link(&ghc, false);
    }
}
