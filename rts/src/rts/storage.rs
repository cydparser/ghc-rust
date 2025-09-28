use std::ptr::null_mut;

use crate::{rts::storage::gc::generation, stg::W_};

pub mod block;
pub mod closure_types;
pub mod closures;
pub mod fun_types;
pub mod gc;
pub mod heap;
pub mod info_tables;
pub mod m_block;
pub mod tso;

#[unsafe(no_mangle)]
pub static mut generations: *mut generation = null_mut();

#[unsafe(no_mangle)]
pub static mut g0: *mut generation = null_mut();

pub(crate) static mut oldest_gen: *mut generation = null_mut();

pub(crate) static mut large_alloc_lim: W_ = 0; // TODO(rust): Set default.

#[unsafe(no_mangle)]
pub static mut keepCAFs: bool = false;
