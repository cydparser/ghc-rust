use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::StgFunPtr;
use crate::prelude::*;

unsafe fn StgRun(mut f: StgFunPtr, mut basereg: *mut StgRegTable) -> *mut StgRegTable {
    let mut r = null_mut::<StgRegTable>();

    asm!(
        "stp x29,  x30,  [sp, #-16]!\n", "\tmov x29, sp\n",
        "\tstp x16, x17, [sp, #-16]!\n", "\tstp x19, x20, [sp, #-16]!\n",
        "\tstp x21, x22, [sp, #-16]!\n", "\tstp x23, x24, [sp, #-16]!\n",
        "\tstp x25, x26, [sp, #-16]!\n", "\tstp x27, x28, [sp, #-16]!\n",
        "\tstp d8,  d9,  [sp, #-16]!\n", "\tstp d10, d11, [sp, #-16]!\n",
        "\tstp d12, d13, [sp, #-16]!\n", "\tstp d14, d15, [sp, #-16]!\n",
        "\tsub sp, sp, {3}\n", "\tmov x19, {2}\n", "\tbr {1}\n", "\t.globl _StgReturn\n",
        "\t_StgReturn:\n", "\tadd sp, sp, {3}\n", "\tmov {0}, x22\n",
        "\tldp d14, d15, [sp], #16\n", "\tldp d12, d13, [sp], #16\n",
        "\tldp d10, d11, [sp], #16\n", "\tldp d8,  d9,  [sp], #16\n",
        "\tldp x27, x28, [sp], #16\n", "\tldp x25, x26, [sp], #16\n",
        "\tldp x23, x24, [sp], #16\n", "\tldp x21, x22, [sp], #16\n",
        "\tldp x19, x20, [sp], #16\n", "\tldp x16, x17, [sp], #16\n",
        "\tldp x29,  x30,  [sp], #16\n", "\t\n", lateout(reg) r, inlateout(reg) f => _,
        inlateout(reg) basereg => _, inlateout(reg) RESERVED_C_STACK_BYTES => _,
        out("x19") _, out("x20") _, out("x21") _, out("x22") _, out("x23") _, out("x24")
        _, out("x25") _, out("x26") _, out("x27") _, out("x28") _, out("x16") _,
        out("x17") _, out("lr") _, options(preserves_flags)
    );

    return r;
}
