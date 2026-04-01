use crate::capability::numa_map;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts::storage::block::MBLOCK_SIZE;
use crate::ffi::rts::storage::heap_alloc::{mblock_address_range, mblock_address_range_Inner};
use crate::ffi::rts::{_assertFail, EXIT_HEAPOVERFLOW, stg_exit};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::sm::os_mem::{
    osBindMBlocksToNode, osCommitMemory, osDecommitMemory, osMemInit, osReleaseHeapMemory,
    osReserveHeapMemory,
};
use crate::trace::{DEBUG_RTS, trace_};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
struct free_list {
    prev: *mut free_list,
    next: *mut free_list,
    address: W_,
    size: W_,
}

static mut peak_mblocks_allocated: W_ = 0;

#[ffi(testsuite)]
#[unsafe(no_mangle)]
pub static mut mblocks_allocated: W_ = 0;

static mut mpc_misses: W_ = 0;

static mut free_list_head: *mut free_list = null_mut::<free_list>();

static mut mblock_high_watermark: W_ = 0;

static mut mblock_address_space: mblock_address_range =
    mblock_address_range(mblock_address_range_Inner {
        begin: 0,
        end: 0,
        padding: [0; 6],
    });

unsafe fn getAllocatedMBlock(
    mut start_iter: *mut *mut free_list,
    mut startingAt: W_,
) -> *mut c_void {
    let mut iter = null_mut::<free_list>();
    let mut p: W_ = startingAt;
    iter = *start_iter;

    while !iter.is_null() {
        if p < (*iter).address {
            break;
        }

        if p == (*iter).address {
            p = p.wrapping_add((*iter).size);
        }

        iter = (*iter).next as *mut free_list;
    }

    *start_iter = iter;

    if p >= mblock_high_watermark {
        return NULL;
    }

    return p as *mut c_void;
}

unsafe fn getFirstMBlock(mut state: *mut *mut c_void) -> *mut c_void {
    let mut fake_state = null_mut::<free_list>();
    let mut casted_state = null_mut::<*mut free_list>();

    if !state.is_null() {
        casted_state = state as *mut *mut free_list;
    } else {
        casted_state = &raw mut fake_state;
    }

    *casted_state = free_list_head;

    return getAllocatedMBlock(casted_state, mblock_address_space.0.begin);
}

unsafe fn getNextMBlock(mut state: *mut *mut c_void, mut mblock: *mut c_void) -> *mut c_void {
    let mut fake_state = free_list_head;
    let mut casted_state = null_mut::<*mut free_list>();

    if !state.is_null() {
        casted_state = state as *mut *mut free_list;
    } else {
        casted_state = &raw mut fake_state;
    }

    return getAllocatedMBlock(casted_state, (mblock as W_).wrapping_add(MBLOCK_SIZE as W_));
}

unsafe fn getReusableMBlocks(mut n: u32) -> *mut c_void {
    let mut iter = null_mut::<free_list>();
    let mut size: W_ = (MBLOCK_SIZE as W_).wrapping_mul(n as W_);
    iter = free_list_head as *mut free_list;

    while !iter.is_null() {
        let mut addr = null_mut::<c_void>();

        if (*iter).size < size {
            iter = (*iter).next;
        } else {
            addr = (*iter).address as *mut c_void;
            (*iter).address = (*iter).address.wrapping_add(size);
            (*iter).size = (*iter).size.wrapping_sub(size);

            if (*iter).size == 0 {
                let mut prev = null_mut::<free_list>();
                let mut next = null_mut::<free_list>();
                prev = (*iter).prev;
                next = (*iter).next;

                if prev.is_null() {
                    if (free_list_head == iter) as i32 as i64 != 0 {
                    } else {
                        _assertFail(c"rts/sm/MBlock.c".as_ptr(), 175);
                    }

                    free_list_head = next as *mut free_list;
                } else {
                    (*prev).next = next;
                }

                if !next.is_null() {
                    (*next).prev = prev;
                }

                stgFree(iter as *mut c_void);
            }

            osCommitMemory(addr, size);

            return addr;
        }
    }

    return NULL;
}

unsafe fn getFreshMBlocks(mut n: u32) -> *mut c_void {
    let mut size: W_ = (MBLOCK_SIZE as W_).wrapping_mul(n as W_);
    let mut addr = mblock_high_watermark as *mut c_void;

    if mblock_high_watermark.wrapping_add(size) > mblock_address_space.0.end {
        errorBelch(c"out of memory".as_ptr());
        stg_exit(EXIT_HEAPOVERFLOW);
    }

    osCommitMemory(addr, size);
    mblock_high_watermark = mblock_high_watermark.wrapping_add(size);

    return addr;
}

unsafe fn getCommittedMBlocks(mut n: u32) -> *mut c_void {
    let mut p = null_mut::<c_void>();
    p = getReusableMBlocks(n);

    if p.is_null() {
        p = getFreshMBlocks(n);
    }

    if (!p.is_null() && p != -1 as *mut c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/MBlock.c".as_ptr(), 219);
    }

    return p;
}

unsafe fn decommitMBlocks(mut addr: *mut c_char, mut n: u32) {
    let mut iter = null_mut::<free_list>();
    let mut prev = null_mut::<free_list>();
    let mut size: W_ = (MBLOCK_SIZE as W_).wrapping_mul(n as W_);
    let mut address: W_ = addr as W_;
    osDecommitMemory(addr as *mut c_void, size);
    prev = null_mut::<free_list>();
    iter = free_list_head as *mut free_list;

    while !iter.is_null() {
        prev = iter;

        if (*iter).address.wrapping_add((*iter).size) < address {
            iter = (*iter).next;
        } else if (*iter).address.wrapping_add((*iter).size) == address {
            (*iter).size = (*iter).size.wrapping_add(size);

            if address.wrapping_add(size) == mblock_high_watermark {
                mblock_high_watermark = mblock_high_watermark.wrapping_sub((*iter).size);

                if !(*iter).prev.is_null() {
                    (*(*iter).prev).next = null_mut::<free_list>();
                } else {
                    if (iter == free_list_head) as i32 as i64 != 0 {
                    } else {
                        _assertFail(c"rts/sm/MBlock.c".as_ptr(), 247);
                    }

                    free_list_head = null_mut::<free_list>();
                }

                stgFree(iter as *mut c_void);
                return;
            }

            if !(*iter).next.is_null()
                && (*(*iter).next).address == (*iter).address.wrapping_add((*iter).size)
            {
                let mut next = null_mut::<free_list>();
                next = (*iter).next;
                (*iter).size = (*iter).size.wrapping_add((*next).size);
                (*iter).next = (*next).next;

                if !(*iter).next.is_null() {
                    (*(*iter).next).prev = iter;

                    if ((*(*iter).next).address > (*iter).address.wrapping_add((*iter).size)) as i32
                        as i64
                        != 0
                    {
                    } else {
                        _assertFail(c"rts/sm/MBlock.c".as_ptr(), 266);
                    }
                }

                stgFree(next as *mut c_void);
            }

            return;
        } else if address.wrapping_add(size) == (*iter).address {
            (*iter).address = address;
            (*iter).size = (*iter).size.wrapping_add(size);

            if !(*iter).prev.is_null() {
                if ((*(*iter).prev).address.wrapping_add((*(*iter).prev).size) < (*iter).address)
                    as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/MBlock.c".as_ptr(), 280);
                }
            }

            return;
        } else {
            let mut new_iter = null_mut::<free_list>();

            if ((*iter).address > address.wrapping_add(size)) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/MBlock.c".as_ptr(), 287);
            }

            new_iter = stgMallocBytes(size_of::<free_list>() as usize, c"freeMBlocks".as_ptr())
                as *mut free_list;

            (*new_iter).address = address;
            (*new_iter).size = size;
            (*new_iter).next = iter;
            (*new_iter).prev = (*iter).prev;

            if !(*new_iter).prev.is_null() {
                (*(*new_iter).prev).next = new_iter;
            } else {
                if (iter == free_list_head) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/MBlock.c".as_ptr(), 297);
                }

                free_list_head = new_iter as *mut free_list;
            }

            (*iter).prev = new_iter;
            return;
        }
    }

    if (address.wrapping_add(size) <= mblock_high_watermark) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/MBlock.c".as_ptr(), 308);
    }

    if address.wrapping_add(size) == mblock_high_watermark {
        mblock_high_watermark = mblock_high_watermark.wrapping_sub(size);
    } else {
        let mut new_iter_0 = null_mut::<free_list>();

        new_iter_0 = stgMallocBytes(size_of::<free_list>() as usize, c"freeMBlocks".as_ptr())
            as *mut free_list;

        (*new_iter_0).address = address;
        (*new_iter_0).size = size;
        (*new_iter_0).next = null_mut::<free_list>();
        (*new_iter_0).prev = prev;

        if !(*new_iter_0).prev.is_null() {
            if (*(*new_iter_0).prev).next.is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/MBlock.c".as_ptr(), 322);
            }

            (*(*new_iter_0).prev).next = new_iter_0;
        } else {
            if free_list_head.is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/MBlock.c".as_ptr(), 325);
            }

            free_list_head = new_iter_0 as *mut free_list;
        }
    };
}

unsafe fn releaseFreeMemory() {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(
            c"mblock_high_watermark: %p\n".as_ptr(),
            mblock_high_watermark,
        );
    }
}

unsafe fn getMBlock() -> *mut c_void {
    return getMBlocks(1);
}

unsafe fn getMBlocks(mut n: u32) -> *mut c_void {
    let mut ret = null_mut::<c_void>();
    ret = getCommittedMBlocks(n);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(c"allocated %d megablock(s) at %p".as_ptr(), n, ret);
    }

    mblocks_allocated = mblocks_allocated.wrapping_add(n as W_);

    peak_mblocks_allocated = ({
        let mut _a: W_ = peak_mblocks_allocated as W_;
        let mut _b: W_ = mblocks_allocated as W_;

        if _a <= _b { _b } else { _a as W_ }
    });

    return ret;
}

unsafe fn getMBlocksOnNode(mut node: u32, mut n: u32) -> *mut c_void {
    let mut addr = getMBlocks(n);

    if RtsFlags.DebugFlags.numa {
        return addr;
    }

    osBindMBlocksToNode(
        addr,
        (n as u64).wrapping_mul(MBLOCK_SIZE) as StgWord,
        numa_map[node as usize],
    );

    return addr;
}

unsafe fn getMBlockOnNode(mut node: u32) -> *mut c_void {
    return getMBlocksOnNode(node, 1);
}

unsafe fn freeMBlocks(mut addr: *mut c_void, mut n: u32) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(c"freeing %d megablock(s) at %p".as_ptr(), n, addr);
    }

    mblocks_allocated = mblocks_allocated.wrapping_sub(n as W_);
    decommitMBlocks(addr as *mut c_char, n);
}

unsafe fn freeAllMBlocks() {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(c"freeing all megablocks".as_ptr());
    }

    let mut iter = null_mut::<free_list>();
    let mut next = null_mut::<free_list>();
    iter = free_list_head as *mut free_list;

    while !iter.is_null() {
        next = (*iter).next;
        stgFree(iter as *mut c_void);
        iter = next;
    }

    osReleaseHeapMemory();
    mblock_address_space.0.begin = -1 as W_;
    mblock_address_space.0.end = -1 as W_;
    mblock_high_watermark = -1 as W_;
}

unsafe fn initMBlocks() {
    osMemInit();

    let mut startAddress = NULL;

    if RtsFlags.GcFlags.heapBase != 0 {
        startAddress = RtsFlags.GcFlags.heapBase as *mut c_void;
    }

    let mut addr = osReserveHeapMemory(startAddress, &raw mut RtsFlags.GcFlags.addressSpaceSize);

    mblock_address_space.0.begin = addr as W_;
    mblock_address_space.0.end = (addr as W_).wrapping_add(RtsFlags.GcFlags.addressSpaceSize as W_);
    mblock_high_watermark = addr as W_;
}
