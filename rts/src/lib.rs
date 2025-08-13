#![allow(clippy::missing_transmute_annotations)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
// TODO: Remove after finishing port.
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

mod prelude;
mod utils;

pub mod hs_ffi;
pub mod mach_deps;
pub mod rts;
pub mod rts_api;
pub mod stg;
