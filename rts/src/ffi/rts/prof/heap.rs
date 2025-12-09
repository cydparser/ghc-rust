use crate::ffi::stg::types::StgWord;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn requestHeapCensus() {
    sys! {
        requestHeapCensus()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startHeapProfTimer() {
    sys! {
        startHeapProfTimer()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stopHeapProfTimer() {
    sys! {
        stopHeapProfTimer()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setUserEra(w: StgWord) {
    sys! {
        setUserEra(w)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getUserEra() -> StgWord {
    sys! {
        getUserEra()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn incrementUserEra(w: StgWord) -> StgWord {
    sys! {
        incrementUserEra(w)
    }
}
