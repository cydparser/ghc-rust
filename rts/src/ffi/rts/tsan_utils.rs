use libc::{c_long, c_short};

use crate::prelude::*;

pub(crate) type __tsan_atomic8 = c_char;

pub(crate) type __tsan_atomic16 = c_short;

pub(crate) type __tsan_atomic32 = c_int;

pub(crate) type __tsan_atomic64 = c_long;
