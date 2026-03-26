use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::storage::block::{
    BF_FRAGMENTED, BF_MARKED, BF_SWEPT, BLOCK_SIZE_W, bdescr, bdescr_, freeGroup,
};
use crate::ffi::rts::storage::gc::{generation, memcount};
use crate::ffi::stg::types::{StgWord, StgWord16};
use crate::ffi::stg::{BITS_PER_BYTE, W_};
use crate::prelude::*;
use crate::trace::{DEBUG_RTS, trace_};

unsafe fn sweep(mut r#gen: *mut generation) {
    let mut bd = null_mut::<bdescr>();
    let mut prev = null_mut::<bdescr>();
    let mut next = null_mut::<bdescr>();
    let mut i: uint32_t = 0;
    let mut freed: W_ = 0;
    let mut resid: W_ = 0;
    let mut fragd: W_ = 0;
    let mut blocks: W_ = 0;
    let mut live: W_ = 0;
    live = 0 as W_;
    freed = 0 as W_;
    fragd = 0 as W_;
    blocks = 0 as W_;
    prev = null_mut::<bdescr>();
    bd = (*r#gen).old_blocks;

    while !bd.is_null() {
        next = (*bd).link as *mut bdescr;

        if (*bd).flags as c_int & BF_MARKED == 0 {
            prev = bd;
        } else {
            blocks = blocks.wrapping_add(1);
            resid = 0 as W_;
            i = 0 as uint32_t;

            while (i as usize)
                < BLOCK_SIZE_W
                    .wrapping_div((BITS_PER_BYTE as usize).wrapping_mul(size_of::<W_>() as usize))
            {
                if *(*bd).u.bitmap.offset(i as isize) != 0 as StgWord {
                    resid = resid.wrapping_add(1);
                }

                i = i.wrapping_add(1);
            }

            live = live.wrapping_add(resid.wrapping_mul(
                (BITS_PER_BYTE as usize).wrapping_mul(size_of::<W_>() as usize) as W_,
            ));

            if resid == 0 as W_ {
                freed = freed.wrapping_add(1);
                (*r#gen).n_old_blocks = (*r#gen).n_old_blocks.wrapping_sub(1);

                if prev.is_null() {
                    (*r#gen).old_blocks = next;
                } else {
                    (*prev).link = next as *mut bdescr_;
                }

                freeGroup(bd);
            } else {
                prev = bd;

                if resid
                    < BLOCK_SIZE_W.wrapping_mul(3 as usize).wrapping_div(
                        (BITS_PER_BYTE as usize)
                            .wrapping_mul(size_of::<W_>() as usize)
                            .wrapping_mul(4 as usize),
                    ) as W_
                {
                    fragd = fragd.wrapping_add(1);
                    (*bd).flags = ((*bd).flags as c_int | BF_FRAGMENTED) as StgWord16;
                }

                (*bd).flags = ((*bd).flags as c_int | BF_SWEPT) as StgWord16;
            }
        }

        bd = next;
    }

    (*r#gen).live_estimate = live as memcount;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
        trace_(
            b"sweeping: %d blocks, %d were copied, %d freed (%d%%), %d are fragmented, live estimate: %ld%%\0"
                as *const u8 as *const c_char as *mut c_char,
            (*r#gen).n_old_blocks.wrapping_add(freed as memcount),
            (*r#gen)
                .n_old_blocks
                .wrapping_sub(blocks as memcount)
                .wrapping_add(freed as memcount),
            freed,
            if blocks == 0 as W_ {
                0 as W_
            } else {
                freed.wrapping_mul(100 as W_).wrapping_div(blocks)
            },
            fragd,
            (if blocks.wrapping_sub(freed) == 0 as W_ {
                0 as W_
            } else {
                live.wrapping_div(BLOCK_SIZE_W as W_)
                    .wrapping_mul(100 as W_)
                    .wrapping_div(blocks.wrapping_sub(freed))
            }) as c_ulong,
        );
    }
}
