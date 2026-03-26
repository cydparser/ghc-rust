use crate::prelude::*;

pub mod hs_ffi;
pub mod mach_deps;
pub mod rts;
pub mod rts_api;
pub mod stg;

pub(crate) const FFI_BAD_ABI: ffi_status = 2;

pub(crate) const FFI_BAD_ARGTYPE: ffi_status = 3;

pub(crate) const FFI_BAD_TYPEDEF: ffi_status = 1;

pub(crate) const FFI_OK: ffi_status = 0;

/// cbindgen:no-export
pub(crate) struct _ffi_type {
    pub(crate) size: size_t,
    pub(crate) alignment: c_ushort,
    pub(crate) r#type: c_ushort,
    pub(crate) elements: *mut *mut _ffi_type,
}

/// cbindgen:no-export
pub(crate) struct ffi_cif {
    pub(crate) abi: ffi_abi,
    pub(crate) nargs: c_uint,
    pub(crate) arg_types: *mut *mut ffi_type,
    pub(crate) rtype: *mut ffi_type,
    pub(crate) bytes: c_uint,
    pub(crate) flags: c_uint,
    pub(crate) aarch64_nfixedargs: c_uint,
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

pub(crate) type ffi_status = c_uint;

pub(crate) type ffi_type = _ffi_type;
