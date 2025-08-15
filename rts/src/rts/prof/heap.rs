use crate::prelude::*;
use crate::stg::types::StgWord;

#[cfg(test)]
mod tests;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_requestHeapCensus"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn requestHeapCensus() {
    unsafe { sys::requestHeapCensus() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_startHeapProfTimer"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn startHeapProfTimer() {
    unsafe { sys::startHeapProfTimer() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stopHeapProfTimer"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stopHeapProfTimer() {
    unsafe { sys::stopHeapProfTimer() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setUserEra"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setUserEra(w: StgWord) {
    unsafe { sys::setUserEra(w) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getUserEra"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getUserEra() -> StgWord {
    unsafe { sys::getUserEra() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_incrementUserEra"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn incrementUserEra(w: StgWord) -> StgWord {
    unsafe { sys::incrementUserEra(w) }
}
