use crate::linker::mach_o_types::{MachORelocationInfo, Stub};
use crate::prelude::*;

static mut instSizeAarch64: size_t = 4 as size_t;

static mut stubSizeAarch64: size_t = (5 as c_int * 4 as c_int) as size_t;

unsafe fn needStubForRelAarch64(mut rel: *mut MachORelocationInfo) -> bool {
    match (*rel).r_type() as c_int {
        2 => return r#true != 0,
        _ => return r#false != 0,
    };
}

unsafe fn makeStubAarch64(mut s: *mut Stub) -> bool {
    let mut mov__hw0_x16: uint32_t = 0xd2800000 as uint32_t | 16 as uint32_t;
    let mut movk_hw0_x16: uint32_t = mov__hw0_x16 | ((1 as c_int) << 29 as c_int) as uint32_t;
    let mut mov__hw3_x16: uint32_t = mov__hw0_x16 | ((3 as c_int) << 21 as c_int) as uint32_t;
    let mut movk_hw2_x16: uint32_t = movk_hw0_x16 | ((2 as c_int) << 21 as c_int) as uint32_t;
    let mut movk_hw1_x16: uint32_t = movk_hw0_x16 | ((1 as c_int) << 21 as c_int) as uint32_t;
    let mut br_x16: uint32_t = 0xd61f0000 as uint32_t | ((16 as c_int) << 5 as c_int) as uint32_t;
    let mut P = (*s).addr as *mut uint32_t;
    let mut addr: uint64_t = (*s).target as uint64_t;
    let mut addr_hw0: uint16_t = (addr >> 0 as c_int) as uint16_t;
    let mut addr_hw1: uint16_t = (addr >> 16 as c_int) as uint16_t;
    let mut addr_hw2: uint16_t = (addr >> 32 as c_int) as uint16_t;
    let mut addr_hw3: uint16_t = (addr >> 48 as c_int) as uint16_t;
    *P.offset(0 as c_int as isize) = mov__hw3_x16 | (addr_hw3 as uint32_t) << 5 as c_int;
    *P.offset(1 as c_int as isize) = movk_hw2_x16 | (addr_hw2 as uint32_t) << 5 as c_int;
    *P.offset(2 as c_int as isize) = movk_hw1_x16 | (addr_hw1 as uint32_t) << 5 as c_int;
    *P.offset(3 as c_int as isize) = movk_hw0_x16 | (addr_hw0 as uint32_t) << 5 as c_int;
    *P.offset(4 as c_int as isize) = br_x16;

    return EXIT_SUCCESS != 0;
}
