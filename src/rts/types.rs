use core::ffi;

use crate::rts::storage::closures::StgClosure_;
use crate::rts::storage::info_tables::StgInfoTable_;
use crate::rts::storage::tso::StgTSO_;

pub type nat = ffi::c_uint;

pub type StgClosure = StgClosure_;

pub type StgInfoTable = StgInfoTable_;

pub type StgTSO = StgTSO_;
