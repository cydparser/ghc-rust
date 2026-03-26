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
    pub(crate) n_active_segs: uint32_t,
    pub(crate) n_filled_segs: uint32_t,
    pub(crate) n_live_blocks: uint32_t,
    pub(crate) n_live_words: uint32_t,
}

unsafe fn nonmovingAllocatorCensus_(
    mut alloc_idx: uint32_t,
    mut collect_live_words: bool,
) -> NonmovingAllocCensus {
    let mut census = NonmovingAllocCensus {
        collected_live_words: collect_live_words,
        n_active_segs: 0 as uint32_t,
        n_filled_segs: 0 as uint32_t,
        n_live_blocks: 0 as uint32_t,
        n_live_words: 0 as uint32_t,
    };

    let mut alloc: *mut NonmovingAllocator =
        nonmovingHeap.allocators.offset(alloc_idx as isize) as *mut NonmovingAllocator;

    let mut seg = (*alloc).filled;

    while !seg.is_null() {
        let mut n = nonmovingSegmentBlockCount(seg);
        census.n_filled_segs = census.n_filled_segs.wrapping_add(1);
        census.n_live_blocks =
            (census.n_live_blocks as c_uint).wrapping_add(n) as uint32_t as uint32_t;

        if collect_live_words {
            let mut i = 0 as c_uint;

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
        let mut i_0 = 0 as c_uint;

        while i_0 < n_0 {
            if nonmovingGetMark(seg_0, i_0 as nonmoving_block_idx) as c_int
                == nonmovingMarkEpoch as c_int
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

    let mut cap_n = 0 as c_uint;

    while cap_n < getNumCapabilities() {
        let mut cap = getCapability(cap_n as uint32_t);
        let mut seg_1 = *(*cap).current_segments.offset(alloc_idx as isize);
        let mut n_1 = nonmovingSegmentBlockCount(seg_1);
        let mut i_1 = 0 as c_uint;

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

unsafe fn nonmovingAllocatorCensusWithWords(mut alloc_idx: uint32_t) -> NonmovingAllocCensus {
    return nonmovingAllocatorCensus_(alloc_idx, r#true != 0);
}

unsafe fn nonmovingAllocatorCensus(mut alloc_idx: uint32_t) -> NonmovingAllocCensus {
    return nonmovingAllocatorCensus_(alloc_idx, r#false != 0);
}

unsafe fn print_alloc_census(mut i: c_int, mut census: NonmovingAllocCensus) {
    let mut blk_size: uint32_t = ((1 as c_int) << i + NONMOVING_ALLOCA0) as uint32_t;
    let mut sz_min = (1 as c_int) << i + NONMOVING_ALLOCA0 - 1 as c_int;
    let mut sz_max = (1 as c_int) << i + NONMOVING_ALLOCA0;

    if census.collected_live_words {
        let mut occupancy =
            100.0f64 * census.n_live_words as c_double * size_of::<W_>() as c_double
                / census.n_live_blocks.wrapping_mul(blk_size) as c_double;
        if census.n_live_blocks == 0 as uint32_t {
            occupancy = 100 as c_int as c_double;
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as c_long != 0 {
            trace_(
                b"Allocator %d (%d bytes - %d bytes): %u active segs, %u filled segs, %u live blocks, %u live words (%2.1f%% occupancy)\0"
                    as *const u8 as *const c_char
                    as *mut c_char,
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
    } else if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as c_long != 0 {
        trace_(
            b"Allocator %d (%d bytes - %d bytes): %u active segs, %u filled segs, %u live blocks\0"
                as *const u8 as *const c_char as *mut c_char,
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

    let mut i = 0 as c_int;

    while i < nonmoving_alloca_cnt as c_int {
        let mut census = nonmovingAllocatorCensus_(i as uint32_t, collect_live_words);
        print_alloc_census(i, census);
        i += 1;
    }
}

unsafe fn nonmovingTraceAllocatorCensus() {
    if !RtsFlags.GcFlags.useNonmoving && TRACE_nonmoving_gc == 0 {
        return;
    }

    let mut i = 0 as c_int;

    while i < nonmoving_alloca_cnt as c_int {
        let census = nonmovingAllocatorCensus(i as uint32_t) as NonmovingAllocCensus;
        let blk_size: uint32_t =
            (*nonmovingHeap.allocators.offset(i as isize)).block_size as uint32_t;
        traceNonmovingHeapCensus(blk_size as uint16_t, &raw const census);
        i += 1;
    }
}
