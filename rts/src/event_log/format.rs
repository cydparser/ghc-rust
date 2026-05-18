//! = Event log format
//!
//! The log format is designed to be extensible: old tools should be
//! able to parse (but not necessarily understand all of) new versions
//! of the format, and new tools will be able to understand old log
//! files.
//!
//! The canonical documentation for the event log format and record layouts is
//! the "Eventlog encodings" section of the GHC User's Guide.
//!
//! == To add a new event
//!
//! - In this file:
//!   - give it a new number, add a new #define EVENT_XXX
//!     below. Do not reuse event ids from deprecated event types.
//!
//! - In EventLog.c
//!   - add it to the EventDesc array
//!   - emit the event type in initEventLogging()
//!   - emit the new event in postEvent_()
//!   - generate the event itself by calling postEvent() somewhere
//!
//! - Describe the meaning and encoding of the event in the users guide
//!   (docs/user_guide/eventlog-formats.rst)
//!
//! - In the Haskell code to parse the event log file:
//!   - add types and code to read the new event

use crate::stg::types::{StgWord16, StgWord32, StgWord64};

macro_rules! marker {
    ($a:literal, $b:literal, $c:literal, $d:literal) => {
        const {
            let mut i: i32 = 0;

            i |= (($a as i32) << 24);
            i |= (($b as i32) << 16);
            i |= (($c as i32) << 8);
            i |= ($d as i32);

            i
        }
    };
}

// Markers for begin/end of the Header.

pub(super) const EVENT_HEADER_BEGIN: StgInt32 = marker!(b'h', b'd', b'r', b'b');
pub(super) const EVENT_HEADER_END: StgInt32 = marker!(b'h', b'd', b'r', b'e');

pub(super) const EVENT_DATA_BEGIN: StgInt32 = marker!(b'd', b'a', b't', b'b');
pub(super) const EVENT_DATA_END: StgInt32 = 0xffff;

// Markers for begin/end of the list of Event Types in the Header.

pub(super) const EVENT_HET_BEGIN: StgInt32 = marker!(b'h', b'e', b't', b'b');
pub(super) const EVENT_HET_END: StgInt32 = marker!(b'h', b'e', b't', b'e');

pub(super) const EVENT_ET_BEGIN: StgInt32 = marker!(b'e', b't', b'b', 0);
pub(super) const EVENT_ET_END: StgInt32 = marker!(b'e', b't', b'e', 0);

// Verify that marker! produces the same ints used in the header file.
const _: () = {
    assert_eq!(EVENT_HEADER_BEGIN, 0x68647262);
    assert_eq!(EVENT_HEADER_END, 0x68647265);
    assert_eq!(EVENT_DATA_BEGIN, 0x64617462);
    assert_eq!(EVENT_DATA_END, 0xffff);
    assert_eq!(EVENT_HET_BEGIN, 0x68657462);
    assert_eq!(EVENT_HET_END, 0x68657465);
    assert_eq!(EVENT_ET_BEGIN, 0x65746200);
    assert_eq!(EVENT_ET_END, 0x65746500);
};

/// The highest event code +1 that ghc itself emits. Note that some event
/// ranges higher than this are reserved but not currently emitted by ghc.
pub(crate) const NUM_GHC_EVENT_TAGS: usize = 213;

// Status values for EVENT_STOP_THREAD

pub(crate) const THREAD_SUSPENDED_FOREIGN_CALL: u32 = 6;

// Capset type values for EVENT_CAPSET_CREATE

/// Reserved for end-user applications
pub(crate) const CAPSET_TYPE_CUSTOM: u32 = 1;
/// Caps belong to the same OS process
pub(crate) const CAPSET_TYPE_OSPROCESS: u32 = 2;
/// Caps share a local clock/time
pub(crate) const CAPSET_TYPE_CLOCKDOMAIN: u32 = 3;

/// Heap profile breakdown types. See `EventTypeNum::HEAP_PROF_BEGIN`.
pub(super) enum HeapProfBreakdown {
    HEAP_PROF_BREAKDOWN_COST_CENTRE = 0x1,
    HEAP_PROF_BREAKDOWN_MODULE,
    HEAP_PROF_BREAKDOWN_CLOSURE_DESCR,
    HEAP_PROF_BREAKDOWN_TYPE_DESCR,
    HEAP_PROF_BREAKDOWN_RETAINER,
    HEAP_PROF_BREAKDOWN_BIOGRAPHY,
    HEAP_PROF_BREAKDOWN_CLOSURE_TYPE,
    HEAP_PROF_BREAKDOWN_INFO_TABLE,
    HEAP_PROF_BREAKDOWN_ERA,
}

/// Nanoseconds
pub(crate) type EventTimestamp = StgWord64;
pub(crate) type EventThreadID = StgWord32;
pub(crate) type EventCapNo = StgWord16;
pub(crate) type EventPayloadSize = StgWord16;
pub(crate) type EventThreadStatus = StgWord16;
pub(crate) type EventCapsetID = StgWord32;
pub(crate) type EventCapsetType = StgWord16;
pub(crate) type EventTaskId = StgWord64;
pub(crate) type EventKernelThreadId = StgWord64;

pub(super) const EVENT_PAYLOAD_SIZE_MAX: usize = StgWord16::MAX as usize;
