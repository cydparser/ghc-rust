use crate::prelude::*;

#[cfg(feature = "header")]
pub mod hs_ffi;
pub mod mach_deps;
pub mod rts;
#[cfg(feature = "header")]
pub mod rts_api;
pub mod stg;

pub(crate) const FFI_BAD_ABI: ffi_status = 2;

pub(crate) const FFI_BAD_ARGTYPE: ffi_status = 3;

pub(crate) const FFI_BAD_TYPEDEF: ffi_status = 1;

pub(crate) const FFI_OK: ffi_status = 0;

/// cbindgen:no-export
pub(crate) struct _ffi_type {
    pub(crate) size: usize,
    pub(crate) alignment: u16,
    pub(crate) r#type: u16,
    pub(crate) elements: *mut *mut _ffi_type,
}

/// cbindgen:no-export
pub(crate) struct ffi_cif {
    pub(crate) abi: ffi_abi,
    pub(crate) nargs: u32,
    pub(crate) arg_types: *mut *mut ffi_type,
    pub(crate) rtype: *mut ffi_type,
    pub(crate) bytes: u32,
    pub(crate) flags: u32,
    pub(crate) aarch64_nfixedargs: u32,
}

/// cbindgen:no-export
pub(crate) struct ffi_closure {
    pub(crate) trampoline_table: *mut c_void,
    pub(crate) trampoline_table_entry: *mut c_void,
    pub(crate) cif: *mut ffi_cif,
    pub(crate) fun: Option<
        unsafe extern "C" fn(*mut ffi_cif, *mut c_void, *mut *mut c_void, *mut c_void) -> (),
    >,
    pub(crate) user_data: *mut c_void,
}

pub(crate) type ffi_status = u32;

pub(crate) type ffi_type = _ffi_type;
