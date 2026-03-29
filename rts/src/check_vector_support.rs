use crate::prelude::*;

unsafe fn checkVectorSupport() -> i32 {
    let mut supports_V16: i32 = 0;
    let mut supports_V32: i32 = 0;
    let mut supports_V64: i32 = 0;
    supports_V16 = 1;
    supports_V32 = 0;
    supports_V64 = 0;

    if supports_V64 != 0 {
        return 3;
    }

    if supports_V32 != 0 {
        return 2;
    }

    if supports_V16 != 0 {
        return 1;
    }

    return 0;
}

static mut vectorSupportGlobalVar: i32 = 0;

unsafe fn setVectorSupport() {
    vectorSupportGlobalVar = checkVectorSupport();
}
