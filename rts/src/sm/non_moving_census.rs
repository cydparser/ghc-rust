use crate::capability::getCapability;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::storage::closure_macros::closure_sizeW;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::StgClosure;
use crate::prelude::*;
use crate::sm::non_moving::{
    NONMOVING_ALLOCA0, NonmovingAllocator, nonmoving_alloca_cnt, nonmoving_block_idx,
    nonmovingGetMark, nonmovingHeap, nonmovingMarkEpoch, nonmovingSegmentBlockCount,
    nonmovingSegmentGetBlock,
};
use crate::sm::non_moving_census::NonmovingAllocCensus;
use crate::trace::{DEBUG_RTS, TRACE_nonmoving_gc, trace_, traceNonmovingHeapCensus};

/// cbindgen:no-export
pub(crate) struct NonmovingAllocCensus {
    pub(crate) collected_live_words: bool,
    pub(crate) n_active_segs: u32,
    pub(crate) n_filled_segs: u32,
    pub(crate) n_live_blocks: u32,
    pub(crate) n_live_words: u32,
}

unsafe fn nonmovingAllocatorCensus_(
    mut alloc_idx: u32,
    mut collect_live_words: bool,
) -> NonmovingAllocCensus {
    let mut census = NonmovingAllocCensus {
        collected_live_words: collect_live_words,
        n_active_segs: 0,
        n_filled_segs: 0,
        n_live_blocks: 0,
        n_live_words: 0,
    };

    let mut alloc: *mut NonmovingAllocator =
        nonmovingHeap.allocators.offset(alloc_idx as isize) as *mut NonmovingAllocator;

    let mut seg = (*alloc).filled;

    while !seg.is_null() {
        let mut n = nonmovingSegmentBlockCount(seg);
        census.n_filled_segs = census.n_filled_segs.wrapping_add(1);
        census.n_live_blocks = (census.n_live_blocks as u32).wrapping_add(n) as u32 as u32;

        if collect_live_words {
            let mut i = 0;

            while i < n {
                let mut c =
                    nonmovingSegmentGetBlock(seg, i as nonmoving_block_idx) as *mut StgClosure;
                census.n_live_words = census.n_live_words.wrapping_add(closure_sizeW(c));
                i = i.wrapping_add(1);
            }
        }

        seg = (*seg).link;
    }

    let mut seg_0 = (*alloc).active;

    while !seg_0.is_null() {
        census.n_active_segs = census.n_active_segs.wrapping_add(1);

        let mut n_0 = nonmovingSegmentBlockCount(seg_0);
        let mut i_0 = 0;

        while i_0 < n_0 {
            if nonmovingGetMark(seg_0, i_0 as nonmoving_block_idx) as i32
                == nonmovingMarkEpoch as i32
            {
                let mut c_0 =
                    nonmovingSegmentGetBlock(seg_0, i_0 as nonmoving_block_idx) as *mut StgClosure;

                if collect_live_words {
                    census.n_live_words = census.n_live_words.wrapping_add(closure_sizeW(c_0));
                }

                census.n_live_blocks = census.n_live_blocks.wrapping_add(1);
            }

            i_0 = i_0.wrapping_add(1);
        }

        seg_0 = (*seg_0).link;
    }

    let mut cap_n = 0;

    while cap_n < getNumCapabilities() {
        let mut cap = getCapability(cap_n as u32);
        let mut seg_1 = *(*cap).current_segments.offset(alloc_idx as isize);
        let mut n_1 = nonmovingSegmentBlockCount(seg_1);
        let mut i_1 = 0;

        while i_1 < n_1 {
            if nonmovingGetMark(seg_1, i_1 as nonmoving_block_idx) != 0 {
                let mut c_1 =
                    nonmovingSegmentGetBlock(seg_1, i_1 as nonmoving_block_idx) as *mut StgClosure;

                if collect_live_words {
                    census.n_live_words = census.n_live_words.wrapping_add(closure_sizeW(c_1));
                }

                census.n_live_blocks = census.n_live_blocks.wrapping_add(1);
            }

            i_1 = i_1.wrapping_add(1);
        }

        cap_n = cap_n.wrapping_add(1);
    }

    return census;
}

unsafe fn nonmovingAllocatorCensusWithWords(mut alloc_idx: u32) -> NonmovingAllocCensus {
    return nonmovingAllocatorCensus_(alloc_idx, true);
}

unsafe fn nonmovingAllocatorCensus(mut alloc_idx: u32) -> NonmovingAllocCensus {
    return nonmovingAllocatorCensus_(alloc_idx, false);
}

unsafe fn print_alloc_census(mut i: i32, mut census: NonmovingAllocCensus) {
    let mut blk_size: u32 = (1 << i + NONMOVING_ALLOCA0) as u32;
    let mut sz_min = 1 << i + NONMOVING_ALLOCA0 - 1;
    let mut sz_max = 1 << i + NONMOVING_ALLOCA0;

    if census.collected_live_words {
        let mut occupancy = 100.0f64 * census.n_live_words as f64 * size_of::<W_>() as f64
            / census.n_live_blocks.wrapping_mul(blk_size) as f64;
        if census.n_live_blocks == 0 {
            occupancy = 100;
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as i64 != 0 {
            trace_(
                c"Allocator %d (%d bytes - %d bytes): %u active segs, %u filled segs, %u live blocks, %u live words (%2.1f%% occupancy)"
                    .as_ptr(),
                i,
                sz_min,
                sz_max,
                census.n_active_segs,
                census.n_filled_segs,
                census.n_live_blocks,
                census.n_live_words,
                occupancy,
            );
        }
    } else if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as i64 != 0 {
        trace_(
            c"Allocator %d (%d bytes - %d bytes): %u active segs, %u filled segs, %u live blocks"
                .as_ptr(),
            i,
            sz_min,
            sz_max,
            census.n_active_segs,
            census.n_filled_segs,
            census.n_live_blocks,
        );
    }
}

unsafe fn nonmovingPrintAllocatorCensus(mut collect_live_words: bool) {
    if !RtsFlags.GcFlags.useNonmoving {
        return;
    }

    let mut i = 0;

    while i < nonmoving_alloca_cnt as i32 {
        let mut census = nonmovingAllocatorCensus_(i as u32, collect_live_words);
        print_alloc_census(i, census);
        i += 1;
    }
}

unsafe fn nonmovingTraceAllocatorCensus() {
    if !RtsFlags.GcFlags.useNonmoving && TRACE_nonmoving_gc == 0 {
        return;
    }

    let mut i = 0;

    while i < nonmoving_alloca_cnt as i32 {
        let census = nonmovingAllocatorCensus(i as u32) as NonmovingAllocCensus;
        let blk_size: u32 = (*nonmovingHeap.allocators.offset(i as isize)).block_size as u32;
        traceNonmovingHeapCensus(blk_size as u16, &raw const census);
        i += 1;
    }
}
