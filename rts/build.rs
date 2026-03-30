use build_utils::{self as utils, Ways};

fn main() {
    if cfg!(feature = "sys") {
        let ghc = utils::GhcConfig::new(Ways {
            threaded: cfg!(feature = "way_threaded"),
            debug: cfg!(feature = "way_debug"),
            profiling: cfg!(feature = "way_profiling"),
            dynamic: true,
        });

        utils::rustc_link(&ghc);
    }
}
