use build_utils::{self as utils, Ways};

fn main() {
    if cfg!(feature = "sys") {
        // TODO: Use features for ways.
        let ghc = utils::GhcConfig::new(Ways {
            threaded: true,
            debug: true,
            profiling: true,
            dynamic: false,
        });

        utils::rustc_link(&ghc);
    }
}
