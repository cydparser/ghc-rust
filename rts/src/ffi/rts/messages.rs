use crate::prelude::*;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rtsOutOfBoundsAccess() -> ! {
    before_exit("rtsOutOfBoundsAccess");
    sys! {
        rtsOutOfBoundsAccess()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rtsMemcpyRangeOverlap() -> ! {
    before_exit("rtsMemcpyRangeOverlap");
    sys! {
        rtsMemcpyRangeOverlap()
    }
}
