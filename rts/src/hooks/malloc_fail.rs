use crate::ffi::stg::W_;
use crate::prelude::*;

unsafe fn MallocFailHook(mut request_size: W_, mut msg: *const c_char) {
    fprintf(
        __stderrp,
        b"malloc: failed on request for %llu bytes; message: %s\n\0" as *const u8 as *const c_char,
        request_size,
        msg,
    );
}
