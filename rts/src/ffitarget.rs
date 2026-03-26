use crate::prelude::*;

pub(crate) const FFI_DEFAULT_ABI: ffi_abi = 1;

pub(crate) const FFI_FIRST_ABI: ffi_abi = 0;

pub(crate) const FFI_LAST_ABI: ffi_abi = 3;

pub(crate) const FFI_SYSV: ffi_abi = 1;

pub(crate) const FFI_WIN64: ffi_abi = 2;

pub(crate) type ffi_abi = c_uint;
