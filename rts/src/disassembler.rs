use crate::ffi::rts::bytecodes::bci_FLAG_LARGE_ARGS;
use crate::ffi::rts::messages::{barf, debugBelch};
use crate::ffi::rts::prof::ccs::CostCentre;
use crate::ffi::rts::storage::closures::StgBCO;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{
    StgDouble, StgFloat, StgInt, StgInt64, StgPtr, StgWord, StgWord8, StgWord16, StgWord32,
    StgWord64,
};
use crate::prelude::*;
use crate::printer::printPtr;

unsafe fn disInstr(mut bco: *mut StgBCO, mut pc: i32) -> i32 {
    let mut instr: StgWord16 = 0;
    let mut instrs = &raw mut (*(*bco).instrs).payload as *mut StgWord as *mut StgWord16;
    let mut literal_arr = (*bco).literals;
    let mut literals = (&raw mut (*literal_arr).payload as *mut StgWord).offset(0) as *mut StgWord;

    let mut ptrs_arr = (*bco).ptrs;
    let mut ptrs = (&raw mut (*ptrs_arr).payload as *mut *mut StgClosure).offset(0)
        as *mut *mut StgClosure as *mut StgPtr;

    let fresh6 = pc;
    pc += 1;
    instr = *instrs.offset(fresh6 as isize);

    if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
        debugBelch(c"LARGE ".as_ptr());
    }

    match instr as i32 & 0xff {
        bci_BRK_FUN => {
            let mut p1: W_ = 0;
            let mut info_mod: W_ = 0;
            let mut info_unit_id: W_ = 0;
            let mut info_wix: W_ = 0;
            let mut np: W_ = 0;

            p1 = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh7 = pc;
                pc = pc + 1;
                *instrs.offset(fresh7 as isize) as StgWord
            }) as W_;

            info_mod = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh8 = pc;
                pc = pc + 1;
                *instrs.offset(fresh8 as isize) as StgWord
            }) as W_;

            info_unit_id = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh9 = pc;
                pc = pc + 1;
                *instrs.offset(fresh9 as isize) as StgWord
            }) as W_;

            pc += 2;
            info_wix = ((*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32)
                .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
                as W_;

            np = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh10 = pc;
                pc = pc + 1;
                *instrs.offset(fresh10 as isize) as StgWord
            }) as W_;

            debugBelch(c"BRK_FUN ".as_ptr());
            printPtr(*ptrs.offset(p1 as isize));
            debugBelch(c" %llu".as_ptr(), *literals.offset(info_mod as isize));
            debugBelch(c" %llu".as_ptr(), *literals.offset(info_unit_id as isize));
            debugBelch(c" %llu".as_ptr(), info_wix);

            let mut cc = *literals.offset(np as isize) as *mut CostCentre;

            if !cc.is_null() {
                debugBelch(c" %s".as_ptr(), (*cc).label);
            }

            debugBelch(c"\n".as_ptr());
        }
        bci_SWIZZLE => {
            let mut stkoff: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh11 = pc;
                pc = pc + 1;
                *instrs.offset(fresh11 as isize) as W_
            };

            let mut by: StgInt = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh12 = pc;
                pc = pc + 1;
                *instrs.offset(fresh12 as isize) as StgWord
            }) as StgInt;

            debugBelch(c"SWIZZLE stkoff %llu by %lld\n".as_ptr(), stkoff, by);
        }
        bci_CCALL => {
            debugBelch(
                c"CCALL    marshaller at 0x%llx\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize),
            );

            pc += 1;
        }
        bci_PRIMCALL => {
            debugBelch(c"PRIMCALL\n".as_ptr());
        }
        bci_STKCHECK => {
            let mut stk_words_reqd: StgWord = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4 as i32;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh13 = pc;
                pc = pc + 1;
                *instrs.offset(fresh13 as isize) as StgWord
            })
            .wrapping_add(1 as StgWord);
            debugBelch(c"STKCHECK %llu\n".as_ptr(), stk_words_reqd);
        }
        bci_PUSH_L => {
            let mut x1: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh14 = pc;
                pc = pc + 1;
                *instrs.offset(fresh14 as isize) as W_
            };

            debugBelch(c"PUSH_L   %llu\n".as_ptr(), x1);
        }
        bci_PUSH_LL => {
            let mut x1_0: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh15 = pc;
                pc = pc + 1;
                *instrs.offset(fresh15 as isize) as W_
            };

            let mut x2: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh16 = pc;
                pc = pc + 1;
                *instrs.offset(fresh16 as isize) as W_
            };

            debugBelch(c"PUSH_LL  %llu %llu\n".as_ptr(), x1_0, x2);
        }
        bci_PUSH_LLL => {
            let mut x1_1: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh17 = pc;
                pc = pc + 1;
                *instrs.offset(fresh17 as isize) as W_
            };

            let mut x2_0: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh18 = pc;
                pc = pc + 1;
                *instrs.offset(fresh18 as isize) as W_
            };

            let mut x3: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh19 = pc;
                pc = pc + 1;
                *instrs.offset(fresh19 as isize) as W_
            };

            debugBelch(c"PUSH_LLL %llu %llu %llu\n".as_ptr(), x1_1, x2_0, x3);
        }
        bci_PUSH8 => {
            let mut x1_2: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh20 = pc;
                pc = pc + 1;
                *instrs.offset(fresh20 as isize) as W_
            };

            debugBelch(c"PUSH8    %llu\n".as_ptr(), x1_2);
        }
        bci_PUSH16 => {
            let mut x1_3: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh21 = pc;
                pc = pc + 1;
                *instrs.offset(fresh21 as isize) as W_
            };

            debugBelch(c"PUSH16   %llu\n".as_ptr(), x1_3);
        }
        bci_PUSH32 => {
            let mut x1_4: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh22 = pc;
                pc = pc + 1;
                *instrs.offset(fresh22 as isize) as W_
            };

            debugBelch(c"PUSH32   %llu\n".as_ptr(), x1_4);
        }
        bci_PUSH8_W => {
            let mut x1_5: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh23 = pc;
                pc = pc + 1;
                *instrs.offset(fresh23 as isize) as W_
            };

            debugBelch(c"PUSH8_W  %llu\n".as_ptr(), x1_5);
        }
        bci_PUSH16_W => {
            let mut x1_6: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh24 = pc;
                pc = pc + 1;
                *instrs.offset(fresh24 as isize) as W_
            };

            debugBelch(c"PUSH16_W %llu\n".as_ptr(), x1_6);
        }
        bci_PUSH32_W => {
            let mut x1_7: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh25 = pc;
                pc = pc + 1;
                *instrs.offset(fresh25 as isize) as W_
            };

            debugBelch(c"PUSH32_W %llu\n".as_ptr(), x1_7);
        }
        bci_PUSH_G => {
            debugBelch(c"PUSH_G   ".as_ptr());
            printPtr(*ptrs.offset(*instrs.offset(pc as isize) as isize));
            debugBelch(c"\n".as_ptr());
            pc += 1;
        }
        bci_PUSH_ALTS_P => {
            debugBelch(c"PUSH_ALTS_P  ".as_ptr());
            printPtr(*ptrs.offset(*instrs.offset(pc as isize) as isize));
            debugBelch(c"\n".as_ptr());
            pc += 1;
        }
        bci_PUSH_ALTS_N => {
            debugBelch(c"PUSH_ALTS_N  ".as_ptr());
            printPtr(*ptrs.offset(*instrs.offset(pc as isize) as isize));
            debugBelch(c"\n".as_ptr());
            pc += 1;
        }
        bci_PUSH_ALTS_F => {
            debugBelch(c"PUSH_ALTS_F  ".as_ptr());
            printPtr(*ptrs.offset(*instrs.offset(pc as isize) as isize));
            debugBelch(c"\n".as_ptr());
            pc += 1;
        }
        bci_PUSH_ALTS_D => {
            debugBelch(c"PUSH_ALTS_D  ".as_ptr());
            printPtr(*ptrs.offset(*instrs.offset(pc as isize) as isize));
            debugBelch(c"\n".as_ptr());
            pc += 1;
        }
        bci_PUSH_ALTS_L => {
            debugBelch(c"PUSH_ALTS_L  ".as_ptr());
            printPtr(*ptrs.offset(*instrs.offset(pc as isize) as isize));
            debugBelch(c"\n".as_ptr());
            pc += 1;
        }
        bci_PUSH_ALTS_V => {
            debugBelch(c"PUSH_ALTS_V  ".as_ptr());
            printPtr(*ptrs.offset(*instrs.offset(pc as isize) as isize));
            debugBelch(c"\n".as_ptr());
            pc += 1;
        }
        bci_PUSH_ALTS_T => {
            debugBelch(c"PUSH_ALTS_T  ".as_ptr());
            printPtr(*ptrs.offset(*instrs.offset(pc as isize) as isize));

            debugBelch(
                c" 0x%llx ".as_ptr(),
                *literals.offset(*instrs.offset((pc + 1) as isize) as isize),
            );

            printPtr(*ptrs.offset(*instrs.offset((pc + 2) as isize) as isize));
            debugBelch(c"\n".as_ptr());
            pc += 3;
        }
        bci_PUSH_PAD8 => {
            debugBelch(c"PUSH_PAD8\n".as_ptr());
            pc += 1;
        }
        bci_PUSH_PAD16 => {
            debugBelch(c"PUSH_PAD16\n".as_ptr());
            pc += 1;
        }
        bci_PUSH_PAD32 => {
            debugBelch(c"PUSH_PAD32\n".as_ptr());
            pc += 1;
        }
        bci_PUSH_UBX8 => {
            debugBelch(
                c"PUSH_UBX8 0x%hhx\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize) as StgWord8 as i32,
            );

            pc += 1;
        }
        bci_PUSH_UBX16 => {
            debugBelch(
                c"PUSH_UBX16 0x%hx\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize) as StgWord16 as i32,
            );

            pc += 1;
        }
        bci_PUSH_UBX32 => {
            debugBelch(
                c"PUSH_UBX32 0x%x\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize) as StgWord32,
            );

            pc += 1;
        }
        bci_PUSH_UBX => {
            debugBelch(c"PUSH_UBX ".as_ptr());

            let mut offset: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh26 = pc;
                pc = pc + 1;
                *instrs.offset(fresh26 as isize) as W_
            };

            let mut nwords: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh27 = pc;
                pc = pc + 1;
                *instrs.offset(fresh27 as isize) as W_
            };

            let mut i: W_ = 0;

            while i < nwords {
                debugBelch(
                    c"0x%llx ".as_ptr(),
                    *literals.offset(i.wrapping_add(offset) as isize),
                );

                i = i.wrapping_add(1);
            }

            debugBelch(c"\n".as_ptr());
        }
        bci_PUSH_APPLY_N => {
            debugBelch(c"PUSH_APPLY_N\n".as_ptr());
        }
        bci_PUSH_APPLY_V => {
            debugBelch(c"PUSH_APPLY_V\n".as_ptr());
        }
        bci_PUSH_APPLY_F => {
            debugBelch(c"PUSH_APPLY_F\n".as_ptr());
        }
        bci_PUSH_APPLY_D => {
            debugBelch(c"PUSH_APPLY_D\n".as_ptr());
        }
        bci_PUSH_APPLY_L => {
            debugBelch(c"PUSH_APPLY_L\n".as_ptr());
        }
        bci_PUSH_APPLY_P => {
            debugBelch(c"PUSH_APPLY_P\n".as_ptr());
        }
        bci_PUSH_APPLY_PP => {
            debugBelch(c"PUSH_APPLY_PP\n".as_ptr());
        }
        bci_PUSH_APPLY_PPP => {
            debugBelch(c"PUSH_APPLY_PPP\n".as_ptr());
        }
        bci_PUSH_APPLY_PPPP => {
            debugBelch(c"PUSH_APPLY_PPPP\n".as_ptr());
        }
        bci_PUSH_APPLY_PPPPP => {
            debugBelch(c"PUSH_APPLY_PPPPP\n".as_ptr());
        }
        bci_PUSH_APPLY_PPPPPP => {
            debugBelch(c"PUSH_APPLY_PPPPPP\n".as_ptr());
        }
        bci_SLIDE => {
            let mut nwords_0: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh28 = pc;
                pc = pc + 1;
                *instrs.offset(fresh28 as isize) as W_
            };

            let mut by_0: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh29 = pc;
                pc = pc + 1;
                *instrs.offset(fresh29 as isize) as W_
            };

            debugBelch(c"SLIDE     %llu down by %llu\n".as_ptr(), nwords_0, by_0);
        }
        bci_ALLOC_AP => {
            let mut nwords_1: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh30 = pc;
                pc = pc + 1;
                *instrs.offset(fresh30 as isize) as W_
            };

            debugBelch(c"ALLOC_AP  %llu words\n".as_ptr(), nwords_1);
        }
        bci_ALLOC_AP_NOUPD => {
            let mut nwords_2: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh31 = pc;
                pc = pc + 1;
                *instrs.offset(fresh31 as isize) as W_
            };

            debugBelch(c"ALLOC_AP_NOUPD %llu words\n".as_ptr(), nwords_2);
        }
        bci_ALLOC_PAP => {
            let mut arity: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh32 = pc;
                pc = pc + 1;
                *instrs.offset(fresh32 as isize) as W_
            };

            let mut nwords_3: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh33 = pc;
                pc = pc + 1;
                *instrs.offset(fresh33 as isize) as W_
            };

            debugBelch(
                c"ALLOC_PAP %llu arity, %llu words\n".as_ptr(),
                arity,
                nwords_3,
            );
        }
        bci_MKAP => {
            let mut stkoff_0: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh34 = pc;
                pc = pc + 1;
                *instrs.offset(fresh34 as isize) as W_
            };

            let mut nwords_4: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh35 = pc;
                pc = pc + 1;
                *instrs.offset(fresh35 as isize) as W_
            };

            debugBelch(
                c"MKAP      %llu words, %llu stkoff\n".as_ptr(),
                nwords_4,
                stkoff_0,
            );
        }
        bci_MKPAP => {
            let mut stkoff_1: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh36 = pc;
                pc = pc + 1;
                *instrs.offset(fresh36 as isize) as W_
            };

            let mut nwords_5: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh37 = pc;
                pc = pc + 1;
                *instrs.offset(fresh37 as isize) as W_
            };

            debugBelch(
                c"MKPAP     %llu words, %llu stkoff\n".as_ptr(),
                nwords_5,
                stkoff_1,
            );
        }
        bci_UNPACK => {
            let mut nwords_6: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh38 = pc;
                pc = pc + 1;
                *instrs.offset(fresh38 as isize) as W_
            };

            debugBelch(c"UNPACK    %llu\n".as_ptr(), nwords_6);
        }
        bci_PACK => {
            let fresh39 = pc;
            pc = pc + 1;

            let mut itbl = *instrs.offset(fresh39 as isize) as i32;

            let mut nwords_7: W_ = if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as W_) << 48 as i32)
                    .wrapping_add((*instrs.offset((pc - 3 as i32) as isize) as W_) << 32 as i32)
                    .wrapping_add((*instrs.offset((pc - 2 as i32) as isize) as W_) << 16 as i32)
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as W_)
            } else {
                let fresh40 = pc;
                pc = pc + 1;
                *instrs.offset(fresh40 as isize) as W_
            };

            debugBelch(c"PACK      %llu words with itbl ".as_ptr(), nwords_7);
            printPtr(*literals.offset(itbl as isize) as StgPtr);
            debugBelch(c"\n".as_ptr());
        }
        bci_TESTLT_I => {
            let fresh41 = pc;
            pc = pc + 1;

            let mut discr = *instrs.offset(fresh41 as isize) as u32;

            let mut failto = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh42 = pc;
                pc = pc + 1;
                *instrs.offset(fresh42 as isize) as StgWord
            }) as i32;

            debugBelch(
                c"TESTLT_I  %lld, fail to %d\n".as_ptr(),
                *literals.offset(discr as isize),
                failto,
            );
        }
        bci_TESTLT_I64 => {
            let fresh43 = pc;
            pc = pc + 1;

            let mut discr_0 = *instrs.offset(fresh43 as isize) as u32;

            let mut failto_0 = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh44 = pc;
                pc = pc + 1;
                *instrs.offset(fresh44 as isize) as StgWord
            }) as i32;

            debugBelch(
                c"TESTLT_I64  %lld, fail to %d\n".as_ptr(),
                *(literals.offset(discr_0 as isize) as *mut StgInt64),
                failto_0,
            );
        }
        bci_TESTLT_I32 => {
            let fresh45 = pc;
            pc = pc + 1;

            let mut discr_1 = *instrs.offset(fresh45 as isize) as u32;

            let mut failto_1 = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh46 = pc;
                pc = pc + 1;
                *instrs.offset(fresh46 as isize) as StgWord
            }) as i32;

            debugBelch(
                c"TESTLT_I32  %lld, fail to %d\n".as_ptr(),
                *literals.offset(discr_1 as isize),
                failto_1,
            );
        }
        bci_TESTLT_I16 => {
            let fresh47 = pc;
            pc = pc + 1;

            let mut discr_2 = *instrs.offset(fresh47 as isize) as u32;

            let mut failto_2 = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh48 = pc;
                pc = pc + 1;
                *instrs.offset(fresh48 as isize) as StgWord
            }) as i32;

            debugBelch(
                c"TESTLT_I16  %lld, fail to %d\n".as_ptr(),
                *literals.offset(discr_2 as isize),
                failto_2,
            );
        }
        bci_TESTLT_I8 => {
            let fresh49 = pc;
            pc = pc + 1;

            let mut discr_3 = *instrs.offset(fresh49 as isize) as u32;

            let mut failto_3 = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh50 = pc;
                pc = pc + 1;
                *instrs.offset(fresh50 as isize) as StgWord
            }) as i32;

            debugBelch(
                c"TESTLT_I8  %lld, fail to %d\n".as_ptr(),
                *literals.offset(discr_3 as isize),
                failto_3,
            );
        }
        bci_TESTEQ_I => {
            debugBelch(
                c"TESTEQ_I  %lld, fail to %d\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_I64 => {
            debugBelch(
                c"TESTEQ_I64  %lld, fail to %d\n".as_ptr(),
                *(literals.offset(*instrs.offset(pc as isize) as i32 as isize) as *mut StgInt64),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_I32 => {
            debugBelch(
                c"TESTEQ_I32  %lld, fail to %d\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_I16 => {
            debugBelch(
                c"TESTEQ_I16  %lld, fail to %d\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_I8 => {
            debugBelch(
                c"TESTEQ_I8  %lld, fail to %d\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTLT_W => {
            let fresh51 = pc;
            pc = pc + 1;

            let mut discr_4 = *instrs.offset(fresh51 as isize) as u32;

            let mut failto_4 = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh52 = pc;
                pc = pc + 1;
                *instrs.offset(fresh52 as isize) as StgWord
            }) as i32;

            debugBelch(
                c"TESTLT_W  %llu, fail to %d\n".as_ptr(),
                *literals.offset(discr_4 as isize),
                failto_4,
            );
        }
        bci_TESTLT_W64 => {
            let fresh53 = pc;
            pc = pc + 1;

            let mut discr_5 = *instrs.offset(fresh53 as isize) as u32;

            let mut failto_5 = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh54 = pc;
                pc = pc + 1;
                *instrs.offset(fresh54 as isize) as StgWord
            }) as i32;

            debugBelch(
                c"TESTLT_W64  %llu, fail to %d\n".as_ptr(),
                *(literals.offset(discr_5 as isize) as *mut StgWord64),
                failto_5,
            );
        }
        bci_TESTLT_W32 => {
            let fresh55 = pc;
            pc = pc + 1;

            let mut discr_6 = *instrs.offset(fresh55 as isize) as u32;

            let mut failto_6 = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh56 = pc;
                pc = pc + 1;
                *instrs.offset(fresh56 as isize) as StgWord
            }) as i32;

            debugBelch(
                c"TESTLT_W32  %llu, fail to %d\n".as_ptr(),
                *literals.offset(discr_6 as isize),
                failto_6,
            );
        }
        bci_TESTLT_W16 => {
            let fresh57 = pc;
            pc = pc + 1;

            let mut discr_7 = *instrs.offset(fresh57 as isize) as u32;

            let mut failto_7 = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh58 = pc;
                pc = pc + 1;
                *instrs.offset(fresh58 as isize) as StgWord
            }) as i32;

            debugBelch(
                c"TESTLT_W16  %llu, fail to %d\n".as_ptr(),
                *literals.offset(discr_7 as isize),
                failto_7,
            );
        }
        bci_TESTLT_W8 => {
            let fresh59 = pc;
            pc = pc + 1;

            let mut discr_8 = *instrs.offset(fresh59 as isize) as u32;

            let mut failto_8 = (if instr as i32 & bci_FLAG_LARGE_ARGS != 0 {
                pc += 4;
                ((*instrs.offset((pc - 4 as i32) as isize) as StgWord) << 48 as i32)
                    .wrapping_add(
                        (*instrs.offset((pc - 3 as i32) as isize) as StgWord) << 32 as i32,
                    )
                    .wrapping_add(
                        (*instrs.offset((pc - 2 as i32) as isize) as StgWord) << 16 as i32,
                    )
                    .wrapping_add(*instrs.offset((pc - 1 as i32) as isize) as StgWord)
            } else {
                let fresh60 = pc;
                pc = pc + 1;
                *instrs.offset(fresh60 as isize) as StgWord
            }) as i32;

            debugBelch(
                c"TESTLT_W8  %llu, fail to %d\n".as_ptr(),
                *literals.offset(discr_8 as isize),
                failto_8,
            );
        }
        bci_TESTEQ_W => {
            debugBelch(
                c"TESTEQ_W  %llu, fail to %d\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_W64 => {
            debugBelch(
                c"TESTEQ_W64  %llu, fail to %d\n".as_ptr(),
                *(literals.offset(*instrs.offset(pc as isize) as i32 as isize) as *mut StgWord64),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_W32 => {
            debugBelch(
                c"TESTEQ_W32  %llu, fail to %d\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_W16 => {
            debugBelch(
                c"TESTEQ_W16  %llu, fail to %d\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_W8 => {
            debugBelch(
                c"TESTEQ_W8  %llu, fail to %d\n".as_ptr(),
                *literals.offset(*instrs.offset(pc as isize) as isize),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTLT_F => {
            debugBelch(
                c"TESTLT_F  %f, fail to %d\n".as_ptr(),
                *(literals as *mut StgFloat).offset(*instrs.offset(pc as isize) as i32 as isize)
                    as f64,
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_F => {
            debugBelch(
                c"TESTEQ_F  %f, fail to %d\n".as_ptr(),
                *(literals as *mut StgFloat).offset(*instrs.offset(pc as isize) as i32 as isize)
                    as f64,
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTLT_D => {
            debugBelch(
                c"TESTLT_D  %f, fail to %d\n".as_ptr(),
                *(literals.offset(*instrs.offset(pc as isize) as i32 as isize) as *mut StgDouble),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_D => {
            debugBelch(
                c"TESTEQ_D  %f, fail to %d\n".as_ptr(),
                *(literals.offset(*instrs.offset(pc as isize) as i32 as isize) as *mut StgDouble),
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTLT_P => {
            debugBelch(
                c"TESTLT_P  %d, fail to %d\n".as_ptr(),
                *instrs.offset(pc as isize) as i32,
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_TESTEQ_P => {
            debugBelch(
                c"TESTEQ_P  %d, fail to %d\n".as_ptr(),
                *instrs.offset(pc as isize) as i32,
                *instrs.offset((pc + 1) as isize) as i32,
            );

            pc += 2;
        }
        bci_CASEFAIL => {
            debugBelch(c"OP_NAME\n".as_ptr());
        }
        bci_JMP => {
            debugBelch(
                c"JMP to    %d\n".as_ptr(),
                *instrs.offset(pc as isize) as i32,
            );
            pc += 1;
        }
        bci_ENTER => {
            debugBelch(c"OP_NAME\n".as_ptr());
        }
        bci_RETURN_P => {
            debugBelch(c"OP_NAME\n".as_ptr());
        }
        bci_RETURN_N => {
            debugBelch(c"OP_NAME\n".as_ptr());
        }
        bci_RETURN_F => {
            debugBelch(c"OP_NAME\n".as_ptr());
        }
        bci_RETURN_D => {
            debugBelch(c"OP_NAME\n".as_ptr());
        }
        bci_RETURN_L => {
            debugBelch(c"OP_NAME\n".as_ptr());
        }
        bci_RETURN_V => {
            debugBelch(c"OP_NAME\n".as_ptr());
        }
        bci_RETURN_T => {
            debugBelch(c"OP_NAME\n".as_ptr());
        }
        bci_BCO_NAME => {
            let mut name = *literals.offset(*instrs.offset(pc as isize) as isize) as *const c_char;
            debugBelch(c"BCO_NAME    \"%s\"\n ".as_ptr(), name);
            pc += 1;
        }
        bci_OP_ADD_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_ADD_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_ADD_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_ADD_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_SUB_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_SUB_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_SUB_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_SUB_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_AND_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_AND_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_AND_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_AND_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_XOR_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_XOR_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_XOR_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_XOR_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_OR_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_OR_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_OR_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_OR_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_NOT_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_NOT_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_NOT_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_NOT_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_NEG_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_NEG_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_NEG_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_NEG_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_MUL_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_MUL_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_MUL_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_MUL_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_SHL_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_SHL_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_SHL_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_SHL_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_ASR_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_ASR_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_ASR_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_ASR_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_LSR_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_LSR_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_LSR_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_LSR_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_NEQ_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_NEQ_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_NEQ_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_NEQ_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_EQ_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_EQ_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_EQ_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_EQ_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_U_GT_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_U_GT_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_U_GT_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_U_GT_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_U_LE_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_U_LE_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_U_LE_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_U_LE_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_U_GE_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_U_GE_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_U_GE_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_U_GE_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_U_LT_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_U_LT_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_U_LT_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_U_LT_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_S_GT_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_S_GT_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_S_GT_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_S_GT_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_S_LE_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_S_LE_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_S_LE_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_S_LE_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_S_GE_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_S_GE_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_S_GE_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_S_GE_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_S_LT_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_S_LT_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_S_LT_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_S_LT_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        bci_OP_INDEX_ADDR_64 => {
            debugBelch(c"#OP_NAME_64\n".as_ptr());
        }
        bci_OP_INDEX_ADDR_32 => {
            debugBelch(c"#OP_NAME_32\n".as_ptr());
        }
        bci_OP_INDEX_ADDR_16 => {
            debugBelch(c"#OP_NAME_16\n".as_ptr());
        }
        bci_OP_INDEX_ADDR_08 => {
            debugBelch(c"#OP_NAME_08\n".as_ptr());
        }
        _ => {
            barf(c"disInstr: unknown opcode %u".as_ptr(), instr as u32);
        }
    }

    return pc;
}

unsafe fn disassemble(mut bco: *mut StgBCO) {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut instrs = &raw mut (*(*bco).instrs).payload as *mut StgWord as *mut StgWord16;
    let mut ptrs = (*bco).ptrs;
    let mut nbcs: u32 = (*(*bco).instrs)
        .bytes
        .wrapping_div(size_of::<StgWord16>() as StgWord) as u32;

    let mut pc: u32 = 0;
    debugBelch(c"BCO\n".as_ptr());

    while pc < nbcs {
        debugBelch(c"\t%2d:  ".as_ptr(), pc);
        pc = disInstr(bco, pc as i32) as u32;
    }

    debugBelch(c"INSTRS:\n   ".as_ptr());
    j = 16;
    i = 0;

    while i < nbcs {
        debugBelch(c"%3d ".as_ptr(), *instrs.offset(i as isize) as i32);
        j = j.wrapping_sub(1);

        if j == 0 {
            j = 16;
            debugBelch(c"\n   ".as_ptr());
        }

        i = i.wrapping_add(1);
    }

    debugBelch(c"\n".as_ptr());
    debugBelch(c"PTRS:\n   ".as_ptr());
    j = 8;
    i = 0;

    while (i as StgWord) < (*ptrs).ptrs {
        debugBelch(
            c"%8p ".as_ptr(),
            *(&raw mut (*ptrs).payload as *mut *mut StgClosure).offset(i as isize),
        );

        j = j.wrapping_sub(1);

        if j == 0 {
            j = 8;
            debugBelch(c"\n   ".as_ptr());
        }

        i = i.wrapping_add(1);
    }

    debugBelch(c"\n".as_ptr());
    debugBelch(c"\n".as_ptr());
}
