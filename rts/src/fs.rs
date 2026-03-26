use crate::prelude::*;

unsafe fn __rts_fopen(mut filename: *const c_char, mut mode: *const c_char) -> *mut FILE {
    return fopen(filename, mode) as *mut FILE;
}
