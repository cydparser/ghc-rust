#![feature(c_variadic)]
#![feature(likely_unlikely)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(clippy::undocumented_unsafe_blocks)]
// TODO(rust): Remove after refactoring.
#![allow(unsafe_op_in_unsafe_fn)]

mod adjustor;
pub(crate) mod capability;
mod config;
mod event_log;
mod os;
mod prelude;
pub(crate) mod rts_flags;
pub(crate) mod rts_startup;
pub(crate) mod time;
mod utils;

pub mod ffi;
