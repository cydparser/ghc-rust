use crate::prelude::*;
use crate::stg::types::StgWord;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn requestHeapCensus() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::requestHeapCensus()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("requestHeapCensus")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startHeapProfTimer() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::startHeapProfTimer()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("startHeapProfTimer")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stopHeapProfTimer() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::stopHeapProfTimer()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("stopHeapProfTimer")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setUserEra(w: StgWord) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::setUserEra(w)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("setUserEra")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getUserEra() -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getUserEra()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getUserEra")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn incrementUserEra(w: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::incrementUserEra(w)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("incrementUserEra")
}
