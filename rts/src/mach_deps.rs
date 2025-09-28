#[cfg(test)]
mod tests;

// TODO(rust): Set constants same way as ghcautoconf.h

/// - GHC_PLACES: {libraries}
pub const SIZEOF_HSINT: u32 = 8;

pub(crate) const ALIGNMENT_HSINT: u32 = 8;

/// - GHC_PLACES: {libraries}
pub const SIZEOF_HSWORD: u32 = 8;

pub(crate) const ALIGNMENT_HSWORD: u32 = 8;

/// - GHC_PLACES: {libraries}
pub const SIZEOF_HSDOUBLE: u32 = 8;

pub(crate) const ALIGNMENT_HSDOUBLE: u32 = 8;

/// - GHC_PLACES: {libraries}
pub const SIZEOF_HSFLOAT: u32 = 4;

pub(crate) const ALIGNMENT_HSFLOAT: u32 = 4;

pub(crate) const SIZEOF_HSPTR: u32 = 8;

pub(crate) const ALIGNMENT_HSPTR: u32 = 8;

pub(crate) const SIZEOF_HSFUNPTR: u32 = 8;

pub(crate) const ALIGNMENT_HSFUNPTR: u32 = 8;

pub(crate) const SIZEOF_HSSTABLEPTR: u32 = 8;

pub(crate) const ALIGNMENT_HSSTABLEPTR: u32 = 8;

pub(crate) const SIZEOF_INT8: u32 = 1;

pub(crate) const ALIGNMENT_INT8: u32 = 1;

pub(crate) const SIZEOF_WORD8: u32 = 1;

pub(crate) const ALIGNMENT_WORD8: u32 = 1;

pub(crate) const SIZEOF_INT16: u32 = 2;

pub(crate) const ALIGNMENT_INT16: u32 = 2;

pub(crate) const SIZEOF_WORD16: u32 = 2;

/// - GHC_PLACES: {libraries}
pub const ALIGNMENT_WORD16: u32 = 2;

pub(crate) const SIZEOF_INT32: u32 = 4;

pub(crate) const ALIGNMENT_INT32: u32 = 4;

pub(crate) const SIZEOF_WORD32: u32 = 4;

pub(crate) const ALIGNMENT_WORD32: u32 = 4;

pub(crate) const SIZEOF_INT64: u32 = 8;

pub(crate) const ALIGNMENT_INT64: u32 = 8;

pub(crate) const SIZEOF_WORD64: u32 = 8;

pub(crate) const ALIGNMENT_WORD64: u32 = 8;

/// - GHC_PLACES: {libraries}
pub const WORD_SIZE_IN_BITS: u32 = 64;

pub(crate) const WORD_SIZE_IN_BITS_FLOAT: f64 = 64.0;

/// - GHC_PLACES: {libraries}
pub const TAG_BITS: u32 = 3;

pub(crate) const TAG_MASK: u32 = 7;
