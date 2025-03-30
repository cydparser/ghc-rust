use crate::rts::storage::{closures::StgClosure_, info_tables::StgInfoTable_, tso::StgTSO_};

#[cfg(test)]
mod tests;

pub type nat = ::core::ffi::c_uint;

pub type StgClosure = StgClosure_;

pub type StgInfoTable = StgInfoTable_;

pub type StgTSO = StgTSO_;
