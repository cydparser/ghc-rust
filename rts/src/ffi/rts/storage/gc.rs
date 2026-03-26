use crate::prelude::*;
pub use crate::schedule::{performBlockingMajorGC, performGC, performMajorGC, setAllocLimitKill};
pub use crate::sm::gc::{generation, generation_, memcount, nursery, nursery_};
pub use crate::sm::gc_aux::revertCAFs;
pub use crate::sm::storage::{
    allocate, dirty_MUT_VAR, g0, generations, keepCAFs, newCAF, setHighMemDynamic, setKeepCAFs,
};

pub(crate) type ListBlocksCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut bdescr)>;

pub(crate) type AdjustorWritable = *mut c_void;

pub(crate) type AdjustorExecutable = *mut c_void;
