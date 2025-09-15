use crate::prelude::*;
use crate::stg::types::StgWord;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_requestHeapCensus"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn requestHeapCensus() {
    unsafe { sys::requestHeapCensus() }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_startHeapProfTimer"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn startHeapProfTimer() {
    unsafe { sys::startHeapProfTimer() }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stopHeapProfTimer"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stopHeapProfTimer() {
    unsafe { sys::stopHeapProfTimer() }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setUserEra"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setUserEra(w: StgWord) {
    unsafe { sys::setUserEra(w) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getUserEra"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getUserEra() -> StgWord {
    unsafe { sys::getUserEra() }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_incrementUserEra"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn incrementUserEra(w: StgWord) -> StgWord {
    unsafe { sys::incrementUserEra(w) }
}
