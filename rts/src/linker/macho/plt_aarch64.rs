use crate::linker::mach_o_types::{MachORelocationInfo, Stub};
use crate::prelude::*;

static mut instSizeAarch64: usize = 4;

static mut stubSizeAarch64: usize = (5 * 4) as usize;

unsafe fn needStubForRelAarch64(mut rel: *mut MachORelocationInfo) -> bool {
    match (*rel).r_type() as i32 {
        2 => return true,
        _ => return false,
    };
}

unsafe fn makeStubAarch64(mut s: *mut Stub) -> bool {
    let mut mov__hw0_x16: u32 = 0xd2800000 | 16;
    let mut movk_hw0_x16: u32 = mov__hw0_x16 | (1 << 29) as u32;
    let mut mov__hw3_x16: u32 = mov__hw0_x16 | (3 << 21) as u32;
    let mut movk_hw2_x16: u32 = movk_hw0_x16 | (2 << 21) as u32;
    let mut movk_hw1_x16: u32 = movk_hw0_x16 | (1 << 21) as u32;
    let mut br_x16: u32 = 0xd61f0000 | (16 << 5) as u32;
    let mut P = (*s).addr as *mut u32;
    let mut addr: u64 = (*s).target as u64;
    let mut addr_hw0: u16 = (addr >> 0) as u16;
    let mut addr_hw1: u16 = (addr >> 16) as u16;
    let mut addr_hw2: u16 = (addr >> 32) as u16;
    let mut addr_hw3: u16 = (addr >> 48) as u16;
    *P.offset(0) = mov__hw3_x16 | (addr_hw3 as u32) << 5;
    *P.offset(1) = movk_hw2_x16 | (addr_hw2 as u32) << 5;
    *P.offset(2) = movk_hw1_x16 | (addr_hw1 as u32) << 5;
    *P.offset(3) = movk_hw0_x16 | (addr_hw0 as u32) << 5;
    *P.offset(4) = br_x16;

    return EXIT_SUCCESS != 0;
}
