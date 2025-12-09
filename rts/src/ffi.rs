#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
// TODO(rust): Remove after finishing port.
#![cfg_attr(not(test), allow(dead_code))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![cfg_attr(not(feature = "sys"), expect(unused_variables))]

pub mod hs_ffi;
pub mod mach_deps;
pub mod rts;
pub mod rts_api;
pub mod stg;
