pub use crate::prim::atomic::{
    hs_atomic_add8, hs_atomic_add16, hs_atomic_add32, hs_atomic_add64, hs_atomic_and8,
    hs_atomic_and16, hs_atomic_and32, hs_atomic_and64, hs_atomic_nand8, hs_atomic_nand16,
    hs_atomic_nand32, hs_atomic_nand64, hs_atomic_or8, hs_atomic_or16, hs_atomic_or32,
    hs_atomic_or64, hs_atomic_sub8, hs_atomic_sub16, hs_atomic_sub32, hs_atomic_sub64,
    hs_atomic_xor8, hs_atomic_xor16, hs_atomic_xor32, hs_atomic_xor64, hs_atomicread8,
    hs_atomicread16, hs_atomicread32, hs_atomicread64, hs_atomicwrite8, hs_atomicwrite16,
    hs_atomicwrite32, hs_atomicwrite64, hs_cmpxchg8, hs_cmpxchg16, hs_cmpxchg32, hs_cmpxchg64,
    hs_xchg8, hs_xchg16, hs_xchg32, hs_xchg64,
};
pub use crate::prim::bitrev::{hs_bitrev8, hs_bitrev16, hs_bitrev32, hs_bitrev64};
pub use crate::prim::bswap::{hs_bswap16, hs_bswap32, hs_bswap64};
pub use crate::prim::clz::{hs_clz8, hs_clz16, hs_clz32, hs_clz64};
pub use crate::prim::ctz::{hs_ctz8, hs_ctz16, hs_ctz32, hs_ctz64};
pub use crate::prim::mul_int_may_oflo::hs_mulIntMayOflo;
pub use crate::prim::pdep::{hs_pdep8, hs_pdep16, hs_pdep32, hs_pdep64};
pub use crate::prim::pext::{hs_pext8, hs_pext16, hs_pext32, hs_pext64};
pub use crate::prim::popcnt::{hs_popcnt, hs_popcnt8, hs_popcnt16, hs_popcnt32, hs_popcnt64};
pub use crate::prim::word2float::{hs_word2float32, hs_word2float64};
