use crate::prelude::*;

// TODO(rust): `barf` is exposed in one place. Stable Rust does not variadic functions.
//   libraries/ghc-internal/cbits/inputReady.c:
//      barf("fdReady: fd is too big: %d but FD_SETSIZE is %d", fd, (int)FD_SETSIZE);

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
