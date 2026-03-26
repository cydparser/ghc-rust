use crate::prelude::*;

pub(crate) type TickProc = Option<unsafe extern "C" fn(c_int) -> ()>;
