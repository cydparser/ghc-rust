use crate::prelude::*;

unsafe fn checkVectorSupport() -> c_int {
    let mut supports_V16: c_int = 0;
    let mut supports_V32: c_int = 0;
    let mut supports_V64: c_int = 0;
    supports_V16 = 1 as c_int;
    supports_V32 = 0 as c_int;
    supports_V64 = 0 as c_int;

    if supports_V64 != 0 {
        return 3 as c_int;
    }

    if supports_V32 != 0 {
        return 2 as c_int;
    }

    if supports_V16 != 0 {
        return 1 as c_int;
    }

    return 0 as c_int;
}

static mut vectorSupportGlobalVar: c_int = 0;

unsafe fn setVectorSupport() {
    vectorSupportGlobalVar = checkVectorSupport();
}
