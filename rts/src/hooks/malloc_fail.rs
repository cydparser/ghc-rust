use crate::ffi::stg::W_;
use crate::prelude::*;

unsafe fn MallocFailHook(mut request_size: W_, mut msg: *const c_char) {
    fprintf(
        __stderrp,
        c"malloc: failed on request for %llu bytes; message: %s\n".as_ptr(),
        request_size,
        msg,
    );
}
