use crate::arena::{Arena, arenaAlloc, arenaFree, newArena};
use crate::clone_stack::{cloneStack, sendCloneStackMessage};
use crate::ffi::hs_ffi::{
    HsBool, HsChar, HsDouble, HsFloat, HsFunPtr, HsInt, HsInt8, HsInt16, HsInt32, HsInt64, HsPtr,
    HsStablePtr, HsWord, HsWord8, HsWord16, HsWord32, HsWord64, hs_exit, hs_exit_nowait,
    hs_free_fun_ptr, hs_free_stable_ptr, hs_free_stable_ptr_unsafe, hs_init,
    hs_lock_stable_ptr_table, hs_lock_stable_tables, hs_perform_gc, hs_set_argv, hs_spt_key_count,
    hs_spt_keys, hs_spt_lookup, hs_thread_done, hs_try_putmvar, hs_try_putmvar_with_value,
    hs_unlock_stable_ptr_table, hs_unlock_stable_tables,
};
use crate::ffi::rts::adjustor::{createAdjustor, freeHaskellFunctionPtr};
use crate::ffi::rts::block_signals::{blockUserSignals, unblockUserSignals};
use crate::ffi::rts::event_log_writer::flushEventLog;
use crate::ffi::rts::exec_page::{ExecPage, allocateExecPage, freeExecPage, freezeExecPage};
use crate::ffi::rts::file_lock::{lockFile, unlockFile};
use crate::ffi::rts::flags::{RTS_FLAGS, RtsFlags};
use crate::ffi::rts::foreign_exports::{ForeignExportsList, registerForeignExports};
use crate::ffi::rts::get_time::getMonotonicNSec;
use crate::ffi::rts::globals::{
    getOrSetGHCConcSignalSignalHandlerStore, getOrSetGHCConcWindowsIOManagerThreadStore,
    getOrSetGHCConcWindowsPendingDelaysStore, getOrSetGHCConcWindowsProddingStore,
    getOrSetLibHSghcFastStringTable, getOrSetLibHSghcGlobalHasNoDebugOutput,
    getOrSetLibHSghcGlobalHasNoStateHack, getOrSetLibHSghcGlobalHasPprDebug,
    getOrSetSystemEventThreadEventManagerStore, getOrSetSystemEventThreadIOManagerThreadStore,
    getOrSetSystemTimerThreadEventManagerStore, getOrSetSystemTimerThreadIOManagerThreadStore,
    ghc_unique_counter64, ghc_unique_inc,
};
use crate::ffi::rts::hpc::{HpcModuleInfo, hs_hpc_module, hs_hpc_rootModule};
use crate::ffi::rts::io_interface::{
    setIOManagerControlFd, setIOManagerWakeupFd, setTimerManagerControlFd,
};
use crate::ffi::rts::ipe::{InfoProvEnt, IpeBufferListNode, lookupIPE, registerInfoProvList};
use crate::ffi::rts::libdw::{
    Backtrace, LibdwSession, Location, backtraceFree, libdwGetBacktrace, libdwLookupLocation,
};
use crate::ffi::rts::libdw_pool::{libdwPoolClear, libdwPoolRelease, libdwPoolTake};
use crate::ffi::rts::linker::{
    addDLL, addLibrarySearchPath, findSystemLibrary, initLinker, initLinker_, insertSymbol,
    loadArchive, loadNativeObj, loadObj, lookupSymbol, lookupSymbolInNativeObj, pathchar, purgeObj,
    removeLibrarySearchPath, resolveObjs, unloadObj,
};
use crate::ffi::rts::non_moving::{
    nonmoving_write_barrier_enabled, stg_copyArray_barrier, updateRemembSetPushClosure_,
    updateRemembSetPushThunk_,
};
use crate::ffi::rts::os_threads::{forkOS_createThread, getNumberOfProcessors};
use crate::ffi::rts::parallel::newSpark;
use crate::ffi::rts::prim_float::{
    __int_encodeDouble, __int_encodeFloat, __word_encodeDouble, __word_encodeFloat,
};
use crate::ffi::rts::prof::ccs::{startProfTimer, stopProfTimer};
use crate::ffi::rts::prof::heap::{
    getUserEra, incrementUserEra, requestHeapCensus, setUserEra, startHeapProfTimer,
    stopHeapProfTimer,
};
use crate::ffi::rts::rts_to_hs_iface::{HsIface, ghc_hs_iface};
use crate::ffi::rts::stable_name::{snEntry, stable_name_table};
use crate::ffi::rts::stable_ptr::{deRefStablePtr, getStablePtr, spEntry, stable_ptr_table};
use crate::ffi::rts::static_ptr_table::{hs_spt_insert, hs_spt_insert_stableptr, hs_spt_remove};
use crate::ffi::rts::storage::closure_macros::closure_sizeW_;
use crate::ffi::rts::storage::closures::{
    StgClosure_, StgIntCharlikeClosure, StgMutVar, StgTVar, StgThunk, StgThunk_, StgWeak,
};
use crate::ffi::rts::storage::gc::{
    allocate, dirty_MUT_VAR, g0, generation, keepCAFs, large_alloc_lim, performBlockingMajorGC,
    performGC, performMajorGC, revertCAFs, setAllocLimitKill, setKeepCAFs,
};
use crate::ffi::rts::storage::info_tables::{closure_flags, stg_arg_bitmaps};
use crate::ffi::rts::storage::tso::{StgStack, StgThreadID};
use crate::ffi::rts::threads::{
    MainCapability, cmp_thread, enabled_capabilities, eq_thread, forkProcess, max_n_capabilities,
    n_capabilities, resumeThread, rts_disableThreadAllocationLimit,
    rts_enableThreadAllocationLimit, rts_getThreadId, rtsSupportsBoundThreads, setNumCapabilities,
    suspendThread,
};
use crate::ffi::rts::ticky::requestTickyCounterSamples;
use crate::ffi::rts::time::{Time, getProcessElapsedTime};
use crate::ffi::rts::timer::{rtsTimerSignal, startTimer, stopTimer};
use crate::ffi::rts::tty::{__hscore_get_saved_termios, __hscore_set_saved_termios};
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::rts::utils::genericRaise;
use crate::ffi::rts::{
    _assertFail, prog_argc, prog_argv, reportHeapOverflow, reportStackOverflow, rts_isDebugged,
    rts_isDynamic, rts_isProfiled, rts_isThreaded, rts_isTracing, stg_exit, stg_sig_install,
};
use crate::ffi::rts_api::{
    Capability, HaskellObj, RTSStats, RtsConfig, defaultRtsConfig, freeFullProgArgv,
    getAllocations, getFullProgArgv, getProgArgv, getRTSStats, getRTSStatsEnabled, hs_init_ghc,
    hs_init_with_rtsopts, rts_apply, rts_checkSchedStatus, rts_clearMemory, rts_eval, rts_eval_,
    rts_evalIO, rts_evalLazyIO, rts_evalStableIO, rts_evalStableIOMain, rts_getBool, rts_getChar,
    rts_getDouble, rts_getFloat, rts_getFunPtr, rts_getInt, rts_getInt8, rts_getInt16,
    rts_getInt32, rts_getInt64, rts_getPtr, rts_getStablePtr, rts_getWord, rts_getWord8,
    rts_getWord16, rts_getWord32, rts_getWord64, rts_inCall, rts_lock, rts_mkBool, rts_mkChar,
    rts_mkDouble, rts_mkFloat, rts_mkFunPtr, rts_mkInt, rts_mkInt8, rts_mkInt16, rts_mkInt32,
    rts_mkInt64, rts_mkPtr, rts_mkStablePtr, rts_mkString, rts_mkWord, rts_mkWord8, rts_mkWord16,
    rts_mkWord32, rts_mkWord64, rts_setInCallCapability, rts_unlock, rts_unsafeGetMyCapability,
    setFullProgArgv, setProgArgv, shutdownHaskell, shutdownHaskellAndExit,
    shutdownHaskellAndSignal, startupHaskell,
};
use crate::ffi::stg::misc_closures::{
    __stg_EAGER_BLACKHOLE_info, __stg_gc_enter_1, __stg_gc_fun, StgReturn, stg_ARR_WORDS_info,
    stg_BLACKHOLE_info, stg_BLOCKING_QUEUE_CLEAN_info, stg_BLOCKING_QUEUE_DIRTY_info,
    stg_CAF_BLACKHOLE_info, stg_CHARLIKE_closure, stg_IND_STATIC_info, stg_INTLIKE_closure,
    stg_MUT_ARR_PTRS_DIRTY_info, stg_MUT_ARR_PTRS_FROZEN_CLEAN_info,
    stg_MUT_ARR_PTRS_FROZEN_DIRTY_info, stg_MUT_VAR_CLEAN_info, stg_MUT_VAR_DIRTY_info,
    stg_MVAR_CLEAN_info, stg_MVAR_DIRTY_info, stg_SMALL_MUT_ARR_PTRS_DIRTY_info,
    stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info, stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info,
    stg_SRT_1_info, stg_SRT_2_info, stg_SRT_3_info, stg_SRT_4_info, stg_SRT_5_info, stg_SRT_6_info,
    stg_SRT_7_info, stg_SRT_8_info, stg_SRT_9_info, stg_SRT_10_info, stg_SRT_11_info,
    stg_SRT_12_info, stg_SRT_13_info, stg_SRT_14_info, stg_SRT_15_info, stg_SRT_16_info,
    stg_TVAR_CLEAN_info, stg_TVAR_DIRTY_info, stg_WEAK_info, stg_absentErrorzh,
    stg_addCFinalizzerToWeakzh, stg_annotateStackzh, stg_ap_0_fast, stg_ap_1_upd_info,
    stg_ap_2_upd_info, stg_ap_3_upd_info, stg_ap_4_upd_info, stg_ap_5_upd_info, stg_ap_6_upd_info,
    stg_ap_7_upd_info, stg_ap_d_fast, stg_ap_d_info, stg_ap_f_fast, stg_ap_f_info, stg_ap_l_fast,
    stg_ap_l_info, stg_ap_n_fast, stg_ap_n_info, stg_ap_p_fast, stg_ap_p_info, stg_ap_pp_fast,
    stg_ap_pp_info, stg_ap_ppp_fast, stg_ap_ppp_info, stg_ap_pppp_fast, stg_ap_pppp_info,
    stg_ap_ppppp_fast, stg_ap_ppppp_info, stg_ap_pppppp_fast, stg_ap_pppppp_info, stg_ap_pppv_fast,
    stg_ap_pppv_info, stg_ap_ppv_fast, stg_ap_ppv_info, stg_ap_pv_fast, stg_ap_pv_info,
    stg_ap_v_fast, stg_ap_v_info, stg_ap_v16_fast, stg_ap_v16_info, stg_ap_v32_fast,
    stg_ap_v32_info, stg_ap_v64_fast, stg_ap_v64_info, stg_atomicModifyMutVar2zh,
    stg_atomicModifyMutVarzuzh, stg_atomicallyzh, stg_bh_upd_frame_info, stg_block_noregs,
    stg_block_putmvar, stg_block_readmvar, stg_block_takemvar, stg_casArrayzh, stg_casInt8Arrayzh,
    stg_casInt16Arrayzh, stg_casInt32Arrayzh, stg_casInt64Arrayzh, stg_casIntArrayzh,
    stg_casMutVarzh, stg_casSmallArrayzh, stg_castDoubleToWord64zh, stg_castFloatToWord32zh,
    stg_castWord32ToFloatzh, stg_castWord64ToDoublezh, stg_catchRetryzh, stg_catchSTMzh,
    stg_catchzh, stg_clearCCSzh, stg_cloneArrayzh, stg_cloneMutableArrayzh, stg_cloneSmallArrayzh,
    stg_cloneSmallMutableArrayzh, stg_closureSizzezh, stg_compactAddWithSharingzh,
    stg_compactAddzh, stg_compactAllocateBlockzh, stg_compactContainsAnyzh, stg_compactContainszh,
    stg_compactFixupPointerszh, stg_compactGetFirstBlockzh, stg_compactGetNextBlockzh,
    stg_compactNewzh, stg_compactResizzezh, stg_compactSizzezh, stg_control0zh, stg_copyArrayzh,
    stg_copyMutableArrayzh, stg_copySmallArrayzh, stg_copySmallMutableArrayzh, stg_ctoi_t,
    stg_deRefStablePtrzh, stg_deRefWeakzh, stg_decodeDoublezu2Intzh, stg_decodeDoublezuInt64zh,
    stg_decodeFloatzuIntzh, stg_delayzh, stg_enter_info, stg_finalizzeWeakzh, stg_forkOnzh,
    stg_forkzh, stg_freezzeArrayzh, stg_freezzeSmallArrayzh, stg_gc_d1, stg_gc_f1, stg_gc_fun_info,
    stg_gc_l1, stg_gc_noregs, stg_gc_pp, stg_gc_ppp, stg_gc_pppp, stg_gc_prim_n, stg_gc_prim_p,
    stg_gc_prim_pp, stg_gc_unbx_r1, stg_gc_unpt_r1, stg_getApStackValzh, stg_getMaskingStatezh,
    stg_getOtherThreadAllocationCounterzh, stg_getSparkzh, stg_getThreadAllocationCounterzh,
    stg_isByteArrayPinnedzh, stg_isByteArrayWeaklyPinnedzh, stg_isCurrentThreadBoundzh,
    stg_isEmptyMVarzh, stg_isMutableByteArrayPinnedzh, stg_isMutableByteArrayWeaklyPinnedzh,
    stg_keepAlivezh, stg_killThreadzh, stg_labelThreadzh, stg_listThreadszh, stg_makeStableNamezh,
    stg_makeStablePtrzh, stg_maskAsyncExceptionszh, stg_maskUninterruptiblezh, stg_mkApUpd0zh,
    stg_mkWeakNoFinalizzerzh, stg_mkWeakzh, stg_newAlignedPinnedByteArrayzh, stg_newArrayzh,
    stg_newBCOzh, stg_newByteArrayzh, stg_newMVarzh, stg_newMutVarzh, stg_newPinnedByteArrayzh,
    stg_newPromptTagzh, stg_newSmallArrayzh, stg_newTVarzh, stg_noDuplicatezh, stg_numSparkszh,
    stg_orig_thunk_info_frame_info, stg_paniczh, stg_primcall_info, stg_promptzh, stg_putMVarzh,
    stg_raiseDivZZerozh, stg_raiseIOzh, stg_raiseOverflowzh, stg_raiseUnderflowzh, stg_raisezh,
    stg_readMVarzh, stg_readTVarIOzh, stg_readTVarzh, stg_resizzeMutableByteArrayzh,
    stg_ret_d_info, stg_ret_f_info, stg_ret_l_info, stg_ret_n_info, stg_ret_p_info, stg_ret_t_info,
    stg_ret_v_info, stg_retryzh, stg_sel_0_noupd_info, stg_sel_0_upd_info, stg_sel_1_noupd_info,
    stg_sel_1_upd_info, stg_sel_2_noupd_info, stg_sel_2_upd_info, stg_sel_3_noupd_info,
    stg_sel_3_upd_info, stg_sel_4_noupd_info, stg_sel_4_upd_info, stg_sel_5_noupd_info,
    stg_sel_5_upd_info, stg_sel_6_noupd_info, stg_sel_6_upd_info, stg_sel_7_noupd_info,
    stg_sel_7_upd_info, stg_sel_8_noupd_info, stg_sel_8_upd_info, stg_sel_9_noupd_info,
    stg_sel_9_upd_info, stg_sel_10_noupd_info, stg_sel_10_upd_info, stg_sel_11_noupd_info,
    stg_sel_11_upd_info, stg_sel_12_noupd_info, stg_sel_12_upd_info, stg_sel_13_noupd_info,
    stg_sel_13_upd_info, stg_sel_14_noupd_info, stg_sel_14_upd_info, stg_sel_15_noupd_info,
    stg_sel_15_upd_info, stg_setOtherThreadAllocationCounterzh, stg_setThreadAllocationCounterzh,
    stg_shrinkMutableByteArrayzh, stg_shrinkSmallMutableArrayzh, stg_takeMVarzh, stg_thawArrayzh,
    stg_thawSmallArrayzh, stg_threadLabelzh, stg_threadStatuszh, stg_traceBinaryEventzh,
    stg_traceEventzh, stg_traceMarkerzh, stg_tryPutMVarzh, stg_tryReadMVarzh, stg_tryTakeMVarzh,
    stg_unmaskAsyncExceptionszh, stg_unpack_cstring_info, stg_unpack_cstring_utf8_info,
    stg_unpackClosurezh, stg_unsafeThawArrayzh, stg_unsafeThawSmallArrayzh, stg_upd_frame_info,
    stg_waitReadzh, stg_waitWritezh, stg_whereFromzh, stg_writeTVarzh, stg_yield_noregs,
    stg_yield_to_interpreter, stg_yieldzh,
};
use crate::ffi::stg::prim::{
    hs_atomic_add8, hs_atomic_add16, hs_atomic_add32, hs_atomic_add64, hs_atomic_and8,
    hs_atomic_and16, hs_atomic_and32, hs_atomic_and64, hs_atomic_nand8, hs_atomic_nand16,
    hs_atomic_nand32, hs_atomic_nand64, hs_atomic_or8, hs_atomic_or16, hs_atomic_or32,
    hs_atomic_or64, hs_atomic_sub8, hs_atomic_sub16, hs_atomic_sub32, hs_atomic_sub64,
    hs_atomic_xor8, hs_atomic_xor16, hs_atomic_xor32, hs_atomic_xor64, hs_atomicread8,
    hs_atomicread16, hs_atomicread32, hs_atomicread64, hs_atomicwrite8, hs_atomicwrite16,
    hs_atomicwrite32, hs_atomicwrite64, hs_bitrev8, hs_bitrev16, hs_bitrev32, hs_bitrev64,
    hs_bswap16, hs_bswap32, hs_bswap64, hs_clz8, hs_clz16, hs_clz32, hs_clz64, hs_cmpxchg8,
    hs_cmpxchg16, hs_cmpxchg32, hs_cmpxchg64, hs_ctz8, hs_ctz16, hs_ctz32, hs_ctz64,
    hs_mulIntMayOflo, hs_pdep8, hs_pdep16, hs_pdep32, hs_pdep64, hs_pext8, hs_pext16, hs_pext32,
    hs_pext64, hs_popcnt, hs_popcnt8, hs_popcnt16, hs_popcnt32, hs_popcnt64, hs_word2float32,
    hs_word2float64, hs_xchg8, hs_xchg16, hs_xchg32, hs_xchg64,
};
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::smp::{atomic_dec, atomic_inc, cas};
use crate::ffi::stg::ticky::{
    ALLOC_BH_adm, ALLOC_BH_ctr, ALLOC_BH_gds, ALLOC_BH_slp, ALLOC_CON_adm, ALLOC_CON_ctr,
    ALLOC_CON_gds, ALLOC_CON_slp, ALLOC_FUN_adm, ALLOC_FUN_ctr, ALLOC_FUN_gds, ALLOC_FUN_slp,
    ALLOC_HEAP_ctr, ALLOC_HEAP_tot, ALLOC_PAP_adm, ALLOC_PAP_ctr, ALLOC_PAP_gds, ALLOC_PAP_slp,
    ALLOC_PRIM_adm, ALLOC_PRIM_ctr, ALLOC_PRIM_gds, ALLOC_PRIM_slp, ALLOC_RTS_ctr, ALLOC_RTS_tot,
    ALLOC_SE_THK_ctr, ALLOC_STACK_ctr, ALLOC_STACK_tot, ALLOC_THK_adm, ALLOC_THK_gds,
    ALLOC_THK_slp, ALLOC_TSO_ctr, ALLOC_TSO_tot, ALLOC_TUP_adm, ALLOC_TUP_ctr, ALLOC_TUP_gds,
    ALLOC_TUP_slp, ALLOC_UP_THK_ctr, CATCHF_PUSHED_ctr, ENT_AP_STACK_ctr, ENT_AP_ctr, ENT_BH_ctr,
    ENT_CONTINUATION_ctr, ENT_DYN_CON_ctr, ENT_DYN_FUN_DIRECT_ctr, ENT_DYN_IND_ctr,
    ENT_DYN_THK_MANY_ctr, ENT_DYN_THK_SINGLE_ctr, ENT_LNE_ctr, ENT_PAP_ctr, ENT_PERM_IND_ctr,
    ENT_STATIC_CON_ctr, ENT_STATIC_FUN_DIRECT_ctr, ENT_STATIC_IND_ctr, ENT_STATIC_THK_MANY_ctr,
    ENT_STATIC_THK_SINGLE_ctr, ENT_VIA_NODE_ctr, GC_FAILED_PROMOTION_ctr, GC_SEL_ABANDONED_ctr,
    GC_SEL_MAJOR_ctr, GC_SEL_MINOR_ctr, HEAP_CHK_ctr, KNOWN_CALL_EXTRA_ARGS_ctr,
    KNOWN_CALL_TOO_FEW_ARGS_ctr, KNOWN_CALL_ctr, MULTI_CHUNK_SLOW_CALL_CHUNKS_ctr,
    MULTI_CHUNK_SLOW_CALL_ctr, RET_NEW_ctr, RET_NEW_hst, RET_OLD_ctr, RET_OLD_hst,
    RET_SEMI_loads_avoided, RET_UNBOXED_TUP_ctr, RET_UNBOXED_TUP_hst, SLOW_CALL_FUN_CORRECT_ctr,
    SLOW_CALL_FUN_TOO_FEW_ctr, SLOW_CALL_FUN_TOO_MANY_ctr, SLOW_CALL_PAP_CORRECT_ctr,
    SLOW_CALL_PAP_TOO_FEW_ctr, SLOW_CALL_PAP_TOO_MANY_ctr, SLOW_CALL_UNEVALD_ctr, SLOW_CALL_ctr,
    SLOW_CALL_fast_d_ctr, SLOW_CALL_fast_f_ctr, SLOW_CALL_fast_l_ctr, SLOW_CALL_fast_n_ctr,
    SLOW_CALL_fast_p_ctr, SLOW_CALL_fast_pp_ctr, SLOW_CALL_fast_ppp_ctr, SLOW_CALL_fast_pppp_ctr,
    SLOW_CALL_fast_ppppp_ctr, SLOW_CALL_fast_pppppp_ctr, SLOW_CALL_fast_pppv_ctr,
    SLOW_CALL_fast_ppv_ctr, SLOW_CALL_fast_pv_ctr, SLOW_CALL_fast_v_ctr, SLOW_CALL_fast_v16_ctr,
    STK_CHK_ctr, TAG_TAGGED_miss, TAG_TAGGED_pred, TAG_UNTAGGED_miss, TAG_UNTAGGED_pred,
    UNKNOWN_CALL_ctr, UPD_CAF_BH_SINGLE_ENTRY_ctr, UPD_CAF_BH_UPDATABLE_ctr, UPD_CON_IN_NEW_ctr,
    UPD_CON_IN_PLACE_ctr, UPD_NEW_IND_ctr, UPD_NEW_PERM_IND_ctr, UPD_OLD_IND_ctr,
    UPD_OLD_PERM_IND_ctr, UPD_PAP_IN_NEW_ctr, UPD_PAP_IN_PLACE_ctr, UPD_SQUEEZED_ctr,
    UPDF_OMITTED_ctr, UPDF_PUSHED_ctr, UPDF_RCC_OMITTED_ctr, UPDF_RCC_PUSHED_ctr,
    VERY_SLOW_CALL_ctr, ticky_slow_call_unevald,
};
use crate::ffi::stg::types::{
    StgDouble, StgFloat, StgFunPtr, StgInt, StgPtr, StgStablePtr, StgVolatilePtr, StgWord,
    StgWord16, StgWord32, StgWord64,
};
use crate::ffi::stg::{I_, W_};
use crate::interpreter::{
    rts_breakpoint_io_action, rts_breakpoint_io_action, rts_disableStopAfterReturn,
    rts_disableStopAfterReturn, rts_disableStopNextBreakpoint, rts_disableStopNextBreakpoint,
    rts_disableStopNextBreakpointAll, rts_disableStopNextBreakpointAll, rts_enableStopAfterReturn,
    rts_enableStopAfterReturn, rts_enableStopNextBreakpoint, rts_enableStopNextBreakpoint,
    rts_enableStopNextBreakpointAll, rts_enableStopNextBreakpointAll, rts_stop_next_breakpoint,
    rts_stop_next_breakpoint, rts_stop_on_exception, rts_stop_on_exception,
};
use crate::posix::signals::signal_handlers;
use crate::posix::signals::{nocldstop, nocldstop};
use crate::prelude::*;
use crate::rts_messages::{
    barf, debugBelch, errorBelch, rtsBadAlignmentBarf, rtsMemcpyRangeOverlap, rtsOutOfBoundsAccess,
    sysErrorBelch,
};
use crate::rts_symbols::{
    _RtsSymbolVal, _SymStrength, _SymType, RtsSymbolVal, STRENGTH_NORMAL, SYM_TYPE_CODE,
    SYM_TYPE_DATA, SymStrength, SymType, SymbolName,
};
use crate::sm::non_moving_mark::updateRemembSetPushThunk;
use crate::sm::storage::dirty_TVAR;
use crate::ticky::{ticky_entry_ctrs, ticky_entry_ctrs, top_ct, top_ct};
use crate::top_handler::rts_setMainThread;

pub(crate) type SymbolAddr = ();

pub(crate) type SymbolName = c_char;

pub(crate) type _SymType = u32;

pub(crate) const SYM_TYPE_HIDDEN: _SymType = 16;

pub(crate) const SYM_TYPE_DUP_DISCARD: _SymType = 8;

pub(crate) const SYM_TYPE_INDIRECT_DATA: _SymType = 4;

pub(crate) const SYM_TYPE_DATA: _SymType = 2;

pub(crate) const SYM_TYPE_CODE: _SymType = 1;

pub(crate) type SymType = _SymType;

pub(crate) type _SymStrength = u32;

pub(crate) const STRENGTH_STRONG: _SymStrength = 2;

pub(crate) const STRENGTH_WEAK: _SymStrength = 1;

pub(crate) const STRENGTH_NORMAL: _SymStrength = 0;

pub(crate) type SymStrength = _SymStrength;

/// cbindgen:no-export
pub(crate) struct _RtsSymbolVal {
    pub(crate) lbl: *const SymbolName,
    pub(crate) addr: *mut c_void,
    pub(crate) strength: SymStrength,
    pub(crate) r#type: SymType,
}

pub(crate) type RtsSymbolVal = _RtsSymbolVal;

extern "C" {
    pub(crate) fn stg_interp_constr7_entry();
    pub(crate) fn stg_interp_constr2_entry();
    pub(crate) fn stg_interp_constr1_entry();
    pub(crate) fn stg_badAlignment_entry();
    pub(crate) fn stg_interp_constr6_entry();
    pub(crate) fn stg_interp_constr3_entry();
    pub(crate) fn stg_interp_constr5_entry();
    pub(crate) fn stg_interp_constr4_entry();
    pub(crate) fn __udivti3();
    pub(crate) fn __umodti3();
    pub(crate) static mut ffi_type_uint64: [StgWord; 0];

    pub(crate) static mut ffi_type_sint16: [StgWord; 0];

    pub(crate) static mut ffi_type_pointer: [StgWord; 0];

    pub(crate) fn ffi_prep_cif();
    pub(crate) fn ffi_call();
    pub(crate) static mut ffi_type_uint8: [StgWord; 0];

    pub(crate) static mut ffi_type_void: [StgWord; 0];

    pub(crate) static mut ffi_type_float: [StgWord; 0];

    pub(crate) static mut ffi_type_double: [StgWord; 0];

    pub(crate) static mut ffi_type_sint64: [StgWord; 0];

    pub(crate) static mut ffi_type_sint8: [StgWord; 0];

    pub(crate) static mut ffi_type_uint16: [StgWord; 0];

    pub(crate) static mut ffi_type_uint32: [StgWord; 0];

    pub(crate) static mut ffi_type_sint32: [StgWord; 0];
}

static mut rtsSyms: [RtsSymbolVal; 736] = unsafe {
    [
        _RtsSymbolVal {
            lbl: b"_stg_mkWeakzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_mkWeakzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_mkWeakNoFinalizzerzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_mkWeakNoFinalizzerzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_addCFinalizzerToWeakzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_addCFinalizzerToWeakzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_makeStableNamezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_makeStableNamezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_finalizzeWeakzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_finalizzeWeakzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ticky_entry_ctrs\0" as *const u8 as *const SymbolName,
            addr: &raw const ticky_entry_ctrs as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_top_ct\0" as *const u8 as *const SymbolName,
            addr: &raw const top_ct as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_VIA_NODE_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_VIA_NODE_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_STATIC_THK_SINGLE_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_STATIC_THK_SINGLE_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_STATIC_THK_MANY_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_STATIC_THK_MANY_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_DYN_THK_SINGLE_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_DYN_THK_SINGLE_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_DYN_THK_MANY_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_DYN_THK_MANY_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_STATIC_FUN_DIRECT_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_STATIC_FUN_DIRECT_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_DYN_FUN_DIRECT_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_DYN_FUN_DIRECT_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_STATIC_CON_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_STATIC_CON_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_DYN_CON_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_DYN_CON_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_STATIC_IND_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_STATIC_IND_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_DYN_IND_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_DYN_IND_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_PERM_IND_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_PERM_IND_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_PAP_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_PAP_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_CONTINUATION_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_CONTINUATION_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_AP_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_AP_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_AP_STACK_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_AP_STACK_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_BH_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_BH_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ENT_LNE_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ENT_LNE_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UNKNOWN_CALL_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UNKNOWN_CALL_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_v16_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_v16_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_v_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_v_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_f_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_f_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_d_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_d_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_l_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_l_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_n_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_n_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_p_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_p_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_pv_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_pv_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_pp_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_pp_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_ppv_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_ppv_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_ppp_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_ppp_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_pppv_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_pppv_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_pppp_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_pppp_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_ppppp_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_ppppp_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_fast_pppppp_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_fast_pppppp_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_VERY_SLOW_CALL_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const VERY_SLOW_CALL_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ticky_slow_call_unevald\0" as *const u8 as *const SymbolName,
            addr: &raw const ticky_slow_call_unevald as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_MULTI_CHUNK_SLOW_CALL_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const MULTI_CHUNK_SLOW_CALL_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_MULTI_CHUNK_SLOW_CALL_CHUNKS_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const MULTI_CHUNK_SLOW_CALL_CHUNKS_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_KNOWN_CALL_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const KNOWN_CALL_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_KNOWN_CALL_TOO_FEW_ARGS_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const KNOWN_CALL_TOO_FEW_ARGS_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_KNOWN_CALL_EXTRA_ARGS_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const KNOWN_CALL_EXTRA_ARGS_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_FUN_TOO_FEW_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_FUN_TOO_FEW_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_FUN_CORRECT_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_FUN_CORRECT_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_FUN_TOO_MANY_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_FUN_TOO_MANY_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_PAP_TOO_FEW_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_PAP_TOO_FEW_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_PAP_CORRECT_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_PAP_CORRECT_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_PAP_TOO_MANY_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_PAP_TOO_MANY_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_SLOW_CALL_UNEVALD_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const SLOW_CALL_UNEVALD_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPDF_OMITTED_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPDF_OMITTED_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPDF_PUSHED_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPDF_PUSHED_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_CATCHF_PUSHED_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const CATCHF_PUSHED_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPDF_RCC_PUSHED_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPDF_RCC_PUSHED_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPDF_RCC_OMITTED_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPDF_RCC_OMITTED_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_SQUEEZED_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_SQUEEZED_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_CON_IN_NEW_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_CON_IN_NEW_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_CON_IN_PLACE_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_CON_IN_PLACE_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_PAP_IN_NEW_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_PAP_IN_NEW_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_PAP_IN_PLACE_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_PAP_IN_PLACE_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_HEAP_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_HEAP_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_HEAP_tot\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_HEAP_tot as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_HEAP_CHK_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const HEAP_CHK_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_STK_CHK_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const STK_CHK_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_RTS_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_RTS_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_RTS_tot\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_RTS_tot as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_FUN_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_FUN_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_FUN_adm\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_FUN_adm as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_FUN_gds\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_FUN_gds as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_FUN_slp\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_FUN_slp as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_NEW_IND_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_NEW_IND_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_NEW_PERM_IND_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_NEW_PERM_IND_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_OLD_IND_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_OLD_IND_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_OLD_PERM_IND_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_OLD_PERM_IND_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_CAF_BH_UPDATABLE_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_CAF_BH_UPDATABLE_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_UPD_CAF_BH_SINGLE_ENTRY_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const UPD_CAF_BH_SINGLE_ENTRY_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_GC_SEL_ABANDONED_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const GC_SEL_ABANDONED_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_GC_SEL_MINOR_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const GC_SEL_MINOR_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_GC_SEL_MAJOR_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const GC_SEL_MAJOR_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_GC_FAILED_PROMOTION_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const GC_FAILED_PROMOTION_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_UP_THK_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_UP_THK_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_SE_THK_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_SE_THK_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_THK_adm\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_THK_adm as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_THK_gds\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_THK_gds as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_THK_slp\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_THK_slp as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_CON_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_CON_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_CON_adm\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_CON_adm as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_CON_gds\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_CON_gds as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_CON_slp\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_CON_slp as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_TUP_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_TUP_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_TUP_adm\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_TUP_adm as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_TUP_gds\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_TUP_gds as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_TUP_slp\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_TUP_slp as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_BH_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_BH_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_BH_adm\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_BH_adm as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_BH_gds\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_BH_gds as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_BH_slp\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_BH_slp as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_PRIM_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_PRIM_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_PRIM_adm\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_PRIM_adm as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_PRIM_gds\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_PRIM_gds as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_PRIM_slp\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_PRIM_slp as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_PAP_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_PAP_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_PAP_adm\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_PAP_adm as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_PAP_gds\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_PAP_gds as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_PAP_slp\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_PAP_slp as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_TSO_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_TSO_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_TSO_tot\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_TSO_tot as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_STACK_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_STACK_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ALLOC_STACK_tot\0" as *const u8 as *const SymbolName,
            addr: &raw const ALLOC_STACK_tot as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_RET_NEW_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const RET_NEW_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_RET_OLD_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const RET_OLD_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_RET_UNBOXED_TUP_ctr\0" as *const u8 as *const SymbolName,
            addr: &raw const RET_UNBOXED_TUP_ctr as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_RET_SEMI_loads_avoided\0" as *const u8 as *const SymbolName,
            addr: &raw const RET_SEMI_loads_avoided as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_TAG_UNTAGGED_pred\0" as *const u8 as *const SymbolName,
            addr: &raw const TAG_UNTAGGED_pred as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_TAG_UNTAGGED_miss\0" as *const u8 as *const SymbolName,
            addr: &raw const TAG_UNTAGGED_miss as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_TAG_TAGGED_pred\0" as *const u8 as *const SymbolName,
            addr: &raw const TAG_TAGGED_pred as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_TAG_TAGGED_miss\0" as *const u8 as *const SymbolName,
            addr: &raw const TAG_TAGGED_miss as *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_RET_NEW_hst\0" as *const u8 as *const SymbolName,
            addr: &raw const RET_NEW_hst as *mut [StgInt; 9] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_RET_OLD_hst\0" as *const u8 as *const SymbolName,
            addr: &raw const RET_OLD_hst as *mut [StgInt; 9] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_RET_UNBOXED_TUP_hst\0" as *const u8 as *const SymbolName,
            addr: &raw const RET_UNBOXED_TUP_hst as *mut [StgInt; 9] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_backtraceFree\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut Backtrace) -> ()>, *mut c_void>(
                Some(backtraceFree as unsafe extern "C" fn(*mut Backtrace) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_libdwGetBacktrace\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut LibdwSession) -> *mut Backtrace>,
                *mut c_void,
            >(Some(
                libdwGetBacktrace as unsafe extern "C" fn(*mut LibdwSession) -> *mut Backtrace,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_libdwLookupLocation\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut LibdwSession, *mut Location, StgPtr) -> c_int>,
                *mut c_void,
            >(Some(
                libdwLookupLocation
                    as unsafe extern "C" fn(*mut LibdwSession, *mut Location, StgPtr) -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_libdwPoolTake\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> *mut LibdwSession>, *mut c_void>(
                Some(libdwPoolTake as unsafe extern "C" fn() -> *mut LibdwSession),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_libdwPoolRelease\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut LibdwSession) -> ()>, *mut c_void>(
                Some(libdwPoolRelease as unsafe extern "C" fn(*mut LibdwSession) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_libdwPoolClear\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                libdwPoolClear as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_StgReturn\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                StgReturn as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ghc_hs_iface\0" as *const u8 as *const SymbolName,
            addr: &raw const ghc_hs_iface as *mut *mut HsIface as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_noregs\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_noregs as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ret_v_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ret_v_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ret_p_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ret_p_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ret_n_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ret_n_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ret_f_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ret_f_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ret_d_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ret_d_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ret_l_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ret_l_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ret_t_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ret_t_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ctoi_t\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ctoi_t as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_primcall_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_primcall_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_prim_p\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_prim_p as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_prim_pp\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_prim_pp as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_prim_n\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_prim_n as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_enter_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_enter_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"___stg_gc_enter_1\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                __stg_gc_enter_1 as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_unpt_r1\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_unpt_r1 as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_unbx_r1\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_unbx_r1 as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_f1\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_f1 as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_d1\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_d1 as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_l1\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_l1 as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_pp\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_pp as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_ppp\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_ppp as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_pppp\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_gc_pppp as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"___stg_gc_fun\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                __stg_gc_fun as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_gc_fun_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_gc_fun_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_yield_noregs\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_yield_noregs as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_yield_to_interpreter\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_yield_to_interpreter as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_block_noregs\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_block_noregs as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_block_takemvar\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_block_takemvar as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_block_readmvar\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_block_readmvar as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_block_putmvar\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_block_putmvar as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_MainCapability\0" as *const u8 as *const SymbolName,
            addr: &raw const MainCapability as *mut Capability as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_addDLL\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut pathchar) -> *const c_char>,
                *mut c_void,
            >(Some(
                addDLL as unsafe extern "C" fn(*mut pathchar) -> *const c_char,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_loadNativeObj\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut pathchar, *mut *mut c_char) -> *mut c_void>,
                *mut c_void,
            >(Some(
                loadNativeObj
                    as unsafe extern "C" fn(*mut pathchar, *mut *mut c_char) -> *mut c_void,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_addLibrarySearchPath\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut pathchar) -> HsPtr>, *mut c_void>(
                Some(addLibrarySearchPath as unsafe extern "C" fn(*mut pathchar) -> HsPtr),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_removeLibrarySearchPath\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HsPtr) -> HsBool>, *mut c_void>(Some(
                removeLibrarySearchPath as unsafe extern "C" fn(HsPtr) -> HsBool,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_findSystemLibrary\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut pathchar) -> *mut pathchar>,
                *mut c_void,
            >(Some(
                findSystemLibrary as unsafe extern "C" fn(*mut pathchar) -> *mut pathchar,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"___int_encodeDouble\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(I_, I_) -> StgDouble>, *mut c_void>(
                Some(__int_encodeDouble as unsafe extern "C" fn(I_, I_) -> StgDouble),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"___word_encodeDouble\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(W_, I_) -> StgDouble>, *mut c_void>(
                Some(__word_encodeDouble as unsafe extern "C" fn(W_, I_) -> StgDouble),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"___int_encodeFloat\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(I_, I_) -> StgFloat>, *mut c_void>(Some(
                __int_encodeFloat as unsafe extern "C" fn(I_, I_) -> StgFloat,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"___word_encodeFloat\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(W_, I_) -> StgFloat>, *mut c_void>(Some(
                __word_encodeFloat as unsafe extern "C" fn(W_, I_) -> StgFloat,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_atomicallyzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_atomicallyzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_barf\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*const c_char, ...) -> !>, *mut c_void>(
                Some(barf as unsafe extern "C" fn(*const c_char, ...) -> !),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_flushEventLog\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut *mut Capability) -> ()>, *mut c_void>(
                Some(flushEventLog as unsafe extern "C" fn(*mut *mut Capability) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_deRefStablePtr\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgStablePtr) -> StgPtr>, *mut c_void>(
                Some(deRefStablePtr as unsafe extern "C" fn(StgStablePtr) -> StgPtr),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_debugBelch\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*const c_char, ...) -> ()>, *mut c_void>(
                Some(debugBelch as unsafe extern "C" fn(*const c_char, ...) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_errorBelch\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*const c_char, ...) -> ()>, *mut c_void>(
                Some(errorBelch as unsafe extern "C" fn(*const c_char, ...) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_sysErrorBelch\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*const c_char, ...) -> ()>, *mut c_void>(
                Some(sysErrorBelch as unsafe extern "C" fn(*const c_char, ...) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_getMaskingStatezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_getMaskingStatezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_maskAsyncExceptionszh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_maskAsyncExceptionszh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_maskUninterruptiblezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_maskUninterruptiblezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_catchzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_catchzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_catchRetryzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_catchRetryzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_catchSTMzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_catchSTMzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_clearCCSzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_clearCCSzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_annotateStackzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_annotateStackzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactAddWithSharingzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactAddWithSharingzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactAddzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactAddzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactNewzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactNewzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactResizzezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactResizzezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactContainszh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactContainszh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactContainsAnyzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactContainsAnyzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactGetFirstBlockzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactGetFirstBlockzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactGetNextBlockzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactGetNextBlockzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactAllocateBlockzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactAllocateBlockzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactFixupPointerszh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactFixupPointerszh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_compactSizzezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_compactSizzezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_closure_flags\0" as *const u8 as *const SymbolName,
            addr: &raw const closure_flags as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_eq_thread\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgPtr, StgPtr) -> bool>, *mut c_void>(
                Some(eq_thread as unsafe extern "C" fn(StgPtr, StgPtr) -> bool),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_cmp_thread\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgPtr, StgPtr) -> c_int>, *mut c_void>(
                Some(cmp_thread as unsafe extern "C" fn(StgPtr, StgPtr) -> c_int),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_createAdjustor\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr, StgFunPtr, *mut c_char) -> *mut c_void>,
                *mut c_void,
            >(Some(
                createAdjustor
                    as unsafe extern "C" fn(StgStablePtr, StgFunPtr, *mut c_char) -> *mut c_void,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_decodeDoublezu2Intzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_decodeDoublezu2Intzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_decodeDoublezuInt64zh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_decodeDoublezuInt64zh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_decodeFloatzuIntzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_decodeFloatzuIntzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_delayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_delayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_deRefWeakzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_deRefWeakzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_deRefStablePtrzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_deRefStablePtrzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_dirty_MUT_VAR\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<
                    unsafe extern "C" fn(*mut StgRegTable, *mut StgMutVar, *mut StgClosure) -> (),
                >,
                *mut c_void,
            >(Some(
                dirty_MUT_VAR
                    as unsafe extern "C" fn(
                        *mut StgRegTable,
                        *mut StgMutVar,
                        *mut StgClosure,
                    ) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_dirty_TVAR\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, *mut StgTVar, *mut StgClosure) -> ()>,
                *mut c_void,
            >(Some(
                dirty_TVAR
                    as unsafe extern "C" fn(*mut Capability, *mut StgTVar, *mut StgClosure) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_forkzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_forkzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_forkOnzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_forkOnzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_forkProcess\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut HsStablePtr) -> pid_t>, *mut c_void>(
                Some(forkProcess as unsafe extern "C" fn(*mut HsStablePtr) -> pid_t),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_forkOS_createThread\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HsStablePtr) -> c_int>, *mut c_void>(
                Some(forkOS_createThread as unsafe extern "C" fn(HsStablePtr) -> c_int),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_freeHaskellFunctionPtr\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut c_void) -> ()>, *mut c_void>(Some(
                freeHaskellFunctionPtr as unsafe extern "C" fn(*mut c_void) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetGHCConcSignalSignalHandlerStore\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetGHCConcSignalSignalHandlerStore
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetGHCConcWindowsPendingDelaysStore\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetGHCConcWindowsPendingDelaysStore
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetGHCConcWindowsIOManagerThreadStore\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetGHCConcWindowsIOManagerThreadStore
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetGHCConcWindowsProddingStore\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetGHCConcWindowsProddingStore
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetSystemEventThreadEventManagerStore\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetSystemEventThreadEventManagerStore
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetSystemEventThreadIOManagerThreadStore\0" as *const u8
                as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetSystemEventThreadIOManagerThreadStore
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetSystemTimerThreadEventManagerStore\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetSystemTimerThreadEventManagerStore
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetSystemTimerThreadIOManagerThreadStore\0" as *const u8
                as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetSystemTimerThreadIOManagerThreadStore
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetLibHSghcFastStringTable\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetLibHSghcFastStringTable
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getRTSStats\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut RTSStats) -> ()>, *mut c_void>(
                Some(getRTSStats as unsafe extern "C" fn(*mut RTSStats) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getRTSStatsEnabled\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> c_int>, *mut c_void>(Some(
                getRTSStatsEnabled as unsafe extern "C" fn() -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetLibHSghcGlobalHasPprDebug\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetLibHSghcGlobalHasPprDebug
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetLibHSghcGlobalHasNoDebugOutput\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetLibHSghcGlobalHasNoDebugOutput
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getOrSetLibHSghcGlobalHasNoStateHack\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgStablePtr) -> StgStablePtr>,
                *mut c_void,
            >(Some(
                getOrSetLibHSghcGlobalHasNoStateHack
                    as unsafe extern "C" fn(StgStablePtr) -> StgStablePtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ghc_unique_counter64\0" as *const u8 as *const SymbolName,
            addr: &raw const ghc_unique_counter64 as *mut HsWord64 as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ghc_unique_inc\0" as *const u8 as *const SymbolName,
            addr: &raw const ghc_unique_inc as *mut HsInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_genericRaise\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int) -> c_int>, *mut c_void>(Some(
                genericRaise as unsafe extern "C" fn(c_int) -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getProgArgv\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> ()>,
                *mut c_void,
            >(Some(
                getProgArgv as unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getFullProgArgv\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> ()>,
                *mut c_void,
            >(Some(
                getFullProgArgv as unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_setFullProgArgv\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> ()>,
                *mut c_void,
            >(Some(
                setFullProgArgv as unsafe extern "C" fn(c_int, *mut *mut c_char) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_freeFullProgArgv\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                freeFullProgArgv as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getProcessElapsedTime\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> Time>, *mut c_void>(Some(
                getProcessElapsedTime as unsafe extern "C" fn() -> Time,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getStablePtr\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgPtr) -> StgStablePtr>, *mut c_void>(
                Some(getStablePtr as unsafe extern "C" fn(StgPtr) -> StgStablePtr),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_registerForeignExports\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut ForeignExportsList) -> ()>,
                *mut c_void,
            >(Some(
                registerForeignExports as unsafe extern "C" fn(*mut ForeignExportsList) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_init\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> ()>,
                *mut c_void,
            >(Some(
                hs_init as unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_init_with_rtsopts\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> ()>,
                *mut c_void,
            >(Some(
                hs_init_with_rtsopts
                    as unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_init_ghc\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char, RtsConfig) -> ()>,
                *mut c_void,
            >(Some(
                hs_init_ghc
                    as unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char, RtsConfig) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_exit\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                hs_exit as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_exit_nowait\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                hs_exit_nowait as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_set_argv\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> ()>,
                *mut c_void,
            >(Some(
                hs_set_argv as unsafe extern "C" fn(c_int, *mut *mut c_char) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_perform_gc\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                hs_perform_gc as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_lock_stable_ptr_table\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                hs_lock_stable_ptr_table as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_unlock_stable_ptr_table\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                hs_unlock_stable_ptr_table as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_lock_stable_tables\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                hs_lock_stable_tables as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_unlock_stable_tables\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                hs_unlock_stable_tables as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_free_stable_ptr\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HsStablePtr) -> ()>, *mut c_void>(Some(
                hs_free_stable_ptr as unsafe extern "C" fn(HsStablePtr) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_free_stable_ptr_unsafe\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HsStablePtr) -> ()>, *mut c_void>(Some(
                hs_free_stable_ptr_unsafe as unsafe extern "C" fn(HsStablePtr) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_free_fun_ptr\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HsFunPtr) -> ()>, *mut c_void>(Some(
                hs_free_fun_ptr as unsafe extern "C" fn(HsFunPtr) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_hpc_rootModule\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> *mut HpcModuleInfo>, *mut c_void>(
                Some(hs_hpc_rootModule as unsafe extern "C" fn() -> *mut HpcModuleInfo),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_hpc_module\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<
                    unsafe extern "C" fn(*mut c_char, StgWord32, StgWord32, *mut StgWord64) -> (),
                >,
                *mut c_void,
            >(Some(
                hs_hpc_module
                    as unsafe extern "C" fn(
                        *mut c_char,
                        StgWord32,
                        StgWord32,
                        *mut StgWord64,
                    ) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_thread_done\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                hs_thread_done as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_try_putmvar\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int, HsStablePtr) -> ()>, *mut c_void>(
                Some(hs_try_putmvar as unsafe extern "C" fn(c_int, HsStablePtr) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_try_putmvar_with_value\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(c_int, HsStablePtr, *mut StgClosure) -> ()>,
                *mut c_void,
            >(Some(
                hs_try_putmvar_with_value
                    as unsafe extern "C" fn(c_int, HsStablePtr, *mut StgClosure) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_defaultRtsConfig\0" as *const u8 as *const SymbolName,
            addr: &raw const defaultRtsConfig as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_initLinker\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                initLinker as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_initLinker_\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int) -> ()>, *mut c_void>(Some(
                initLinker_ as unsafe extern "C" fn(c_int) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_unpackClosurezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_unpackClosurezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_closureSizzezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_closureSizzezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_whereFromzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_whereFromzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_getApStackValzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_getApStackValzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_getSparkzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_getSparkzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_numSparkszh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_numSparkszh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_isCurrentThreadBoundzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_isCurrentThreadBoundzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_isEmptyMVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_isEmptyMVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_killThreadzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_killThreadzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_listThreadszh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_listThreadszh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_threadLabelzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_threadLabelzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_loadArchive\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut pathchar) -> HsInt>, *mut c_void>(
                Some(loadArchive as unsafe extern "C" fn(*mut pathchar) -> HsInt),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_loadObj\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut pathchar) -> HsInt>, *mut c_void>(
                Some(loadObj as unsafe extern "C" fn(*mut pathchar) -> HsInt),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_purgeObj\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut pathchar) -> HsInt>, *mut c_void>(
                Some(purgeObj as unsafe extern "C" fn(*mut pathchar) -> HsInt),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_insertSymbol\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut pathchar, *mut c_char, *mut c_void) -> HsInt>,
                *mut c_void,
            >(Some(
                insertSymbol
                    as unsafe extern "C" fn(*mut pathchar, *mut c_char, *mut c_void) -> HsInt,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_lookupSymbol\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut c_char) -> *mut c_void>, *mut c_void>(
                Some(lookupSymbol as unsafe extern "C" fn(*mut c_char) -> *mut c_void),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_lookupSymbolInNativeObj\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void>,
                *mut c_void,
            >(Some(
                lookupSymbolInNativeObj
                    as unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_makeStablePtrzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_makeStablePtrzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_mkApUpd0zh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_mkApUpd0zh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_labelThreadzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_labelThreadzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_newArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_newArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_copyArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_copyArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_copyMutableArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_copyMutableArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_cloneArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_cloneArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_cloneMutableArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_cloneMutableArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_freezzeArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_freezzeArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_thawArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_thawArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_casArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_casArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_newSmallArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_newSmallArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_unsafeThawSmallArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_unsafeThawSmallArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_cloneSmallArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_cloneSmallArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_cloneSmallMutableArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_cloneSmallMutableArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_freezzeSmallArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_freezzeSmallArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_thawSmallArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_thawSmallArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_copySmallArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_copySmallArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_copySmallMutableArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_copySmallMutableArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_casSmallArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_casSmallArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_copyArray_barrier\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_copyArray_barrier as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_newBCOzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_newBCOzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_newByteArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_newByteArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_casIntArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_casIntArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_casInt8Arrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_casInt8Arrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_casInt16Arrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_casInt16Arrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_casInt32Arrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_casInt32Arrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_casInt64Arrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_casInt64Arrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_newMVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_newMVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_newMutVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_newMutVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_newTVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_newTVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_noDuplicatezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_noDuplicatezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_atomicModifyMutVar2zh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_atomicModifyMutVar2zh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_atomicModifyMutVarzuzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_atomicModifyMutVarzuzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_casMutVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_casMutVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_newPinnedByteArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_newPinnedByteArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_newAlignedPinnedByteArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_newAlignedPinnedByteArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_isByteArrayPinnedzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_isByteArrayPinnedzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_isMutableByteArrayPinnedzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_isMutableByteArrayPinnedzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_isByteArrayWeaklyPinnedzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_isByteArrayWeaklyPinnedzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_isMutableByteArrayWeaklyPinnedzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_isMutableByteArrayWeaklyPinnedzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_shrinkMutableByteArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_shrinkMutableByteArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_resizzeMutableByteArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_resizzeMutableByteArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_shrinkSmallMutableArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_shrinkSmallMutableArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_newSpark\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut StgRegTable, *mut StgClosure) -> StgInt>,
                *mut c_void,
            >(Some(
                newSpark as unsafe extern "C" fn(*mut StgRegTable, *mut StgClosure) -> StgInt,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_updateRemembSetPushThunk\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, *mut StgThunk) -> ()>,
                *mut c_void,
            >(Some(
                updateRemembSetPushThunk
                    as unsafe extern "C" fn(*mut Capability, *mut StgThunk) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_updateRemembSetPushThunk_\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut StgRegTable, *mut StgThunk_) -> ()>,
                *mut c_void,
            >(Some(
                updateRemembSetPushThunk_
                    as unsafe extern "C" fn(*mut StgRegTable, *mut StgThunk_) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_updateRemembSetPushClosure_\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut StgRegTable, *mut StgClosure_) -> ()>,
                *mut c_void,
            >(Some(
                updateRemembSetPushClosure_
                    as unsafe extern "C" fn(*mut StgRegTable, *mut StgClosure_) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_performGC\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                performGC as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_performMajorGC\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                performMajorGC as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_performBlockingMajorGC\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                performBlockingMajorGC as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_prog_argc\0" as *const u8 as *const SymbolName,
            addr: &raw const prog_argc as *mut i32 as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_prog_argv\0" as *const u8 as *const SymbolName,
            addr: &raw const prog_argv as *mut *mut *mut c_char as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_putMVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_putMVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_raisezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_raisezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_raiseDivZZerozh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_raiseDivZZerozh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_raiseUnderflowzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_raiseUnderflowzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_raiseOverflowzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_raiseOverflowzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_raiseIOzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_raiseIOzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_keepAlivezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_keepAlivezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_paniczh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_paniczh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_absentErrorzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_absentErrorzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_readTVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_readTVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_readTVarIOzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_readTVarIOzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_resumeThread\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut c_void) -> *mut StgRegTable>,
                *mut c_void,
            >(Some(
                resumeThread as unsafe extern "C" fn(*mut c_void) -> *mut StgRegTable,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_setNumCapabilities\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_uint) -> ()>, *mut c_void>(Some(
                setNumCapabilities as unsafe extern "C" fn(c_uint) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getNumberOfProcessors\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> c_uint>, *mut c_void>(Some(
                getNumberOfProcessors as unsafe extern "C" fn() -> c_uint,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_resolveObjs\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> HsInt>, *mut c_void>(Some(
                resolveObjs as unsafe extern "C" fn() -> HsInt,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_retryzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_retryzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_rts_apply\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HaskellObj, HaskellObj) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_apply
                    as unsafe extern "C" fn(*mut Capability, HaskellObj, HaskellObj) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_checkSchedStatus\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut c_char, *mut Capability) -> ()>,
                *mut c_void,
            >(Some(
                rts_checkSchedStatus as unsafe extern "C" fn(*mut c_char, *mut Capability) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_eval\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<
                    unsafe extern "C" fn(*mut *mut Capability, HaskellObj, *mut HaskellObj) -> (),
                >,
                *mut c_void,
            >(Some(
                rts_eval
                    as unsafe extern "C" fn(
                        *mut *mut Capability,
                        HaskellObj,
                        *mut HaskellObj,
                    ) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_evalIO\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<
                    unsafe extern "C" fn(*mut *mut Capability, HaskellObj, *mut HaskellObj) -> (),
                >,
                *mut c_void,
            >(Some(
                rts_evalIO
                    as unsafe extern "C" fn(
                        *mut *mut Capability,
                        HaskellObj,
                        *mut HaskellObj,
                    ) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_evalLazyIO\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<
                    unsafe extern "C" fn(*mut *mut Capability, HaskellObj, *mut HaskellObj) -> (),
                >,
                *mut c_void,
            >(Some(
                rts_evalLazyIO
                    as unsafe extern "C" fn(
                        *mut *mut Capability,
                        HaskellObj,
                        *mut HaskellObj,
                    ) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_evalStableIOMain\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<
                    unsafe extern "C" fn(*mut *mut Capability, HsStablePtr, *mut HsStablePtr) -> (),
                >,
                *mut c_void,
            >(Some(
                rts_evalStableIOMain
                    as unsafe extern "C" fn(
                        *mut *mut Capability,
                        HsStablePtr,
                        *mut HsStablePtr,
                    ) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_evalStableIO\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<
                    unsafe extern "C" fn(*mut *mut Capability, HsStablePtr, *mut HsStablePtr) -> (),
                >,
                *mut c_void,
            >(Some(
                rts_evalStableIO
                    as unsafe extern "C" fn(
                        *mut *mut Capability,
                        HsStablePtr,
                        *mut HsStablePtr,
                    ) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_eval_\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut Capability,
                        HaskellObj,
                        c_uint,
                        *mut HaskellObj,
                    ) -> (),
                >,
                *mut c_void,
            >(Some(
                rts_eval_
                    as unsafe extern "C" fn(
                        *mut *mut Capability,
                        HaskellObj,
                        c_uint,
                        *mut HaskellObj,
                    ) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_inCall\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<
                    unsafe extern "C" fn(*mut *mut Capability, HaskellObj, *mut HaskellObj) -> (),
                >,
                *mut c_void,
            >(Some(
                rts_inCall
                    as unsafe extern "C" fn(
                        *mut *mut Capability,
                        HaskellObj,
                        *mut HaskellObj,
                    ) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getBool\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsBool>, *mut c_void>(
                Some(rts_getBool as unsafe extern "C" fn(HaskellObj) -> HsBool),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getChar\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsChar>, *mut c_void>(
                Some(rts_getChar as unsafe extern "C" fn(HaskellObj) -> HsChar),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getDouble\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsDouble>, *mut c_void>(
                Some(rts_getDouble as unsafe extern "C" fn(HaskellObj) -> HsDouble),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getFloat\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsFloat>, *mut c_void>(
                Some(rts_getFloat as unsafe extern "C" fn(HaskellObj) -> HsFloat),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getInt\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsInt>, *mut c_void>(
                Some(rts_getInt as unsafe extern "C" fn(HaskellObj) -> HsInt),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getInt8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsInt8>, *mut c_void>(
                Some(rts_getInt8 as unsafe extern "C" fn(HaskellObj) -> HsInt8),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getInt16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsInt16>, *mut c_void>(
                Some(rts_getInt16 as unsafe extern "C" fn(HaskellObj) -> HsInt16),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getInt32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsInt32>, *mut c_void>(
                Some(rts_getInt32 as unsafe extern "C" fn(HaskellObj) -> HsInt32),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getInt64\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsInt64>, *mut c_void>(
                Some(rts_getInt64 as unsafe extern "C" fn(HaskellObj) -> HsInt64),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getPtr\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsPtr>, *mut c_void>(
                Some(rts_getPtr as unsafe extern "C" fn(HaskellObj) -> HsPtr),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getFunPtr\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsFunPtr>, *mut c_void>(
                Some(rts_getFunPtr as unsafe extern "C" fn(HaskellObj) -> HsFunPtr),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getStablePtr\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsStablePtr>, *mut c_void>(
                Some(rts_getStablePtr as unsafe extern "C" fn(HaskellObj) -> HsStablePtr),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getThreadId\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgPtr) -> StgThreadID>, *mut c_void>(
                Some(rts_getThreadId as unsafe extern "C" fn(StgPtr) -> StgThreadID),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getWord\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsWord>, *mut c_void>(
                Some(rts_getWord as unsafe extern "C" fn(HaskellObj) -> HsWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getWord8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsWord8>, *mut c_void>(
                Some(rts_getWord8 as unsafe extern "C" fn(HaskellObj) -> HsWord8),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getWord16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsWord16>, *mut c_void>(
                Some(rts_getWord16 as unsafe extern "C" fn(HaskellObj) -> HsWord16),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getWord32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsWord32>, *mut c_void>(
                Some(rts_getWord32 as unsafe extern "C" fn(HaskellObj) -> HsWord32),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_getWord64\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(HaskellObj) -> HsWord64>, *mut c_void>(
                Some(rts_getWord64 as unsafe extern "C" fn(HaskellObj) -> HsWord64),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_lock\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> *mut Capability>, *mut c_void>(
                Some(rts_lock as unsafe extern "C" fn() -> *mut Capability),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkBool\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsBool) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkBool as unsafe extern "C" fn(*mut Capability, HsBool) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkChar\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsChar) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkChar as unsafe extern "C" fn(*mut Capability, HsChar) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkDouble\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsDouble) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkDouble as unsafe extern "C" fn(*mut Capability, HsDouble) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkFloat\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsFloat) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkFloat as unsafe extern "C" fn(*mut Capability, HsFloat) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkInt\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsInt) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkInt as unsafe extern "C" fn(*mut Capability, HsInt) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkInt8\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsInt8) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkInt8 as unsafe extern "C" fn(*mut Capability, HsInt8) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkInt16\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsInt16) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkInt16 as unsafe extern "C" fn(*mut Capability, HsInt16) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkInt32\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsInt32) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkInt32 as unsafe extern "C" fn(*mut Capability, HsInt32) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkInt64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsInt64) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkInt64 as unsafe extern "C" fn(*mut Capability, HsInt64) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkPtr\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsPtr) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkPtr as unsafe extern "C" fn(*mut Capability, HsPtr) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkFunPtr\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsFunPtr) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkFunPtr as unsafe extern "C" fn(*mut Capability, HsFunPtr) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkStablePtr\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsStablePtr) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkStablePtr as unsafe extern "C" fn(*mut Capability, HsStablePtr) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkString\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, *mut c_char) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkString as unsafe extern "C" fn(*mut Capability, *mut c_char) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkWord\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsWord) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkWord as unsafe extern "C" fn(*mut Capability, HsWord) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkWord8\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsWord8) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkWord8 as unsafe extern "C" fn(*mut Capability, HsWord8) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkWord16\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsWord16) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkWord16 as unsafe extern "C" fn(*mut Capability, HsWord16) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkWord32\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsWord32) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkWord32 as unsafe extern "C" fn(*mut Capability, HsWord32) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_mkWord64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, HsWord64) -> HaskellObj>,
                *mut c_void,
            >(Some(
                rts_mkWord64 as unsafe extern "C" fn(*mut Capability, HsWord64) -> HaskellObj,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_unlock\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut Capability) -> ()>, *mut c_void>(
                Some(rts_unlock as unsafe extern "C" fn(*mut Capability) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_unsafeGetMyCapability\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> *mut Capability>, *mut c_void>(
                Some(rts_unsafeGetMyCapability as unsafe extern "C" fn() -> *mut Capability),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rtsSupportsBoundThreads\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> HsBool>, *mut c_void>(Some(
                rtsSupportsBoundThreads as unsafe extern "C" fn() -> HsBool,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_isProfiled\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> c_int>, *mut c_void>(Some(
                rts_isProfiled as unsafe extern "C" fn() -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_isDynamic\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> c_int>, *mut c_void>(Some(
                rts_isDynamic as unsafe extern "C" fn() -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_isThreaded\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> c_int>, *mut c_void>(Some(
                rts_isThreaded as unsafe extern "C" fn() -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_isDebugged\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> c_int>, *mut c_void>(Some(
                rts_isDebugged as unsafe extern "C" fn() -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_isTracing\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> c_int>, *mut c_void>(Some(
                rts_isTracing as unsafe extern "C" fn() -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_setInCallCapability\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int, c_int) -> ()>, *mut c_void>(Some(
                rts_setInCallCapability as unsafe extern "C" fn(c_int, c_int) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_enableThreadAllocationLimit\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgPtr) -> ()>, *mut c_void>(Some(
                rts_enableThreadAllocationLimit as unsafe extern "C" fn(StgPtr) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_disableThreadAllocationLimit\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgPtr) -> ()>, *mut c_void>(Some(
                rts_disableThreadAllocationLimit as unsafe extern "C" fn(StgPtr) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_setMainThread\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut StgWeak) -> ()>, *mut c_void>(Some(
                rts_setMainThread as unsafe extern "C" fn(*mut StgWeak) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_setAllocLimitKill\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(bool, bool) -> ()>, *mut c_void>(Some(
                setAllocLimitKill as unsafe extern "C" fn(bool, bool) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_setProgArgv\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> ()>,
                *mut c_void,
            >(Some(
                setProgArgv as unsafe extern "C" fn(c_int, *mut *mut c_char) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_startupHaskell\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<
                    unsafe extern "C" fn(
                        c_int,
                        *mut *mut c_char,
                        Option<unsafe extern "C" fn() -> ()>,
                    ) -> (),
                >,
                *mut c_void,
            >(Some(
                startupHaskell
                    as unsafe extern "C" fn(
                        c_int,
                        *mut *mut c_char,
                        Option<unsafe extern "C" fn() -> ()>,
                    ) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_shutdownHaskell\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                shutdownHaskell as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_shutdownHaskellAndExit\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int, c_int) -> !>, *mut c_void>(Some(
                shutdownHaskellAndExit as unsafe extern "C" fn(c_int, c_int) -> !,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stable_name_table\0" as *const u8 as *const SymbolName,
            addr: &raw const stable_name_table as *mut *mut snEntry as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stable_ptr_table\0" as *const u8 as *const SymbolName,
            addr: &raw const stable_ptr_table as *mut *mut spEntry as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_reportStackOverflow\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut StgTSO) -> ()>, *mut c_void>(Some(
                reportStackOverflow as unsafe extern "C" fn(*mut StgTSO) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_reportHeapOverflow\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                reportHeapOverflow as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_CAF_BLACKHOLE_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_CAF_BLACKHOLE_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_BLACKHOLE_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_BLACKHOLE_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"___stg_EAGER_BLACKHOLE_info\0" as *const u8 as *const SymbolName,
            addr: &raw const __stg_EAGER_BLACKHOLE_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_BLOCKING_QUEUE_CLEAN_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_BLOCKING_QUEUE_CLEAN_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_BLOCKING_QUEUE_DIRTY_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_BLOCKING_QUEUE_DIRTY_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_startTimer\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                startTimer as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_MVAR_CLEAN_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_MVAR_CLEAN_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_MVAR_DIRTY_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_MVAR_DIRTY_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_TVAR_CLEAN_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_TVAR_CLEAN_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_TVAR_DIRTY_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_TVAR_DIRTY_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_IND_STATIC_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_IND_STATIC_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ARR_WORDS_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ARR_WORDS_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_MUT_ARR_PTRS_DIRTY_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_MUT_ARR_PTRS_DIRTY_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_MUT_ARR_PTRS_FROZEN_CLEAN_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_MUT_ARR_PTRS_FROZEN_DIRTY_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_MUT_ARR_PTRS_FROZEN_DIRTY_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SMALL_MUT_ARR_PTRS_DIRTY_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SMALL_MUT_ARR_PTRS_DIRTY_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_MUT_VAR_CLEAN_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_MUT_VAR_CLEAN_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_MUT_VAR_DIRTY_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_MUT_VAR_DIRTY_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_WEAK_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_WEAK_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_1_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_1_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_2_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_2_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_3_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_3_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_4_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_4_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_5_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_5_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_6_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_6_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_7_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_7_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_8_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_8_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_9_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_9_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_10_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_10_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_11_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_11_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_12_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_12_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_13_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_13_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_14_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_14_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_15_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_15_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_SRT_16_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_SRT_16_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_v_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_v_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_f_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_f_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_d_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_d_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_l_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_l_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_v16_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_v16_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_v32_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_v32_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_v64_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_v64_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_n_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_n_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_p_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_p_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_pv_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_pv_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_pp_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_pp_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_ppv_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_ppv_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_ppp_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_ppp_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_pppv_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_pppv_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_pppp_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_pppp_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_ppppp_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_ppppp_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_pppppp_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_pppppp_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_0_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_0_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_v_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_v_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_f_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_f_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_d_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_d_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_l_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_l_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_v16_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_v16_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_v32_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_v32_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_v64_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_v64_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_n_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_n_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_p_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_p_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_pv_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_pv_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_pp_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_pp_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_ppv_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_ppv_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_ppp_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_ppp_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_pppv_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_pppv_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_pppp_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_pppp_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_ppppp_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_ppppp_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_pppppp_fast\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_ap_pppppp_fast as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_1_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_1_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_2_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_2_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_3_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_3_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_4_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_4_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_5_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_5_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_6_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_6_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_ap_7_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_ap_7_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_exit\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int) -> !>, *mut c_void>(Some(
                stg_exit as unsafe extern "C" fn(c_int) -> !,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_0_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_0_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_1_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_1_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_2_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_2_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_3_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_3_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_4_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_4_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_5_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_5_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_6_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_6_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_7_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_7_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_8_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_8_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_9_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_9_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_10_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_10_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_11_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_11_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_12_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_12_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_13_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_13_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_14_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_14_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_15_upd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_15_upd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_0_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_0_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_1_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_1_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_2_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_2_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_3_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_3_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_4_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_4_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_5_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_5_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_6_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_6_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_7_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_7_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_8_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_8_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_9_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_9_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_10_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_10_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_11_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_11_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_12_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_12_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_13_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_13_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_14_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_14_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sel_15_noupd_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_sel_15_noupd_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_unpack_cstring_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_unpack_cstring_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_unpack_cstring_utf8_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_unpack_cstring_utf8_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_upd_frame_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_upd_frame_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_bh_upd_frame_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_bh_upd_frame_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_orig_thunk_info_frame_info\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_orig_thunk_info_frame_info as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_suspendThread\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut StgRegTable, bool) -> *mut c_void>,
                *mut c_void,
            >(Some(
                suspendThread as unsafe extern "C" fn(*mut StgRegTable, bool) -> *mut c_void,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_takeMVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_takeMVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_readMVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_readMVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_threadStatuszh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_threadStatuszh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_tryPutMVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_tryPutMVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_tryTakeMVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_tryTakeMVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_tryReadMVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_tryReadMVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_unmaskAsyncExceptionszh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_unmaskAsyncExceptionszh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_unloadObj\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut pathchar) -> HsInt>, *mut c_void>(
                Some(unloadObj as unsafe extern "C" fn(*mut pathchar) -> HsInt),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_unsafeThawArrayzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_unsafeThawArrayzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_waitReadzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_waitReadzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_waitWritezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_waitWritezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_writeTVarzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_writeTVarzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_yieldzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_yieldzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_badAlignment_entry\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stg_badAlignment_entry as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_interp_constr1_entry\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stg_interp_constr1_entry as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_interp_constr2_entry\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stg_interp_constr2_entry as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_interp_constr3_entry\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stg_interp_constr3_entry as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_interp_constr4_entry\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stg_interp_constr4_entry as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_interp_constr5_entry\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stg_interp_constr5_entry as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_interp_constr6_entry\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stg_interp_constr6_entry as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_interp_constr7_entry\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stg_interp_constr7_entry as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_arg_bitmaps\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_arg_bitmaps as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_large_alloc_lim\0" as *const u8 as *const SymbolName,
            addr: &raw const large_alloc_lim as *mut W_ as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_g0\0" as *const u8 as *const SymbolName,
            addr: &raw const g0 as *mut *mut generation as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_allocate\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, W_) -> StgPtr>,
                *mut c_void,
            >(Some(
                allocate as unsafe extern "C" fn(*mut Capability, W_) -> StgPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_allocateExecPage\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> *mut ExecPage>, *mut c_void>(Some(
                allocateExecPage as unsafe extern "C" fn() -> *mut ExecPage,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_freezeExecPage\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut ExecPage) -> ()>, *mut c_void>(
                Some(freezeExecPage as unsafe extern "C" fn(*mut ExecPage) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_freeExecPage\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut ExecPage) -> ()>, *mut c_void>(
                Some(freeExecPage as unsafe extern "C" fn(*mut ExecPage) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getAllocations\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> c_ulong>, *mut c_void>(Some(
                getAllocations as unsafe extern "C" fn() -> c_ulong,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_revertCAFs\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                revertCAFs as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_RtsFlags\0" as *const u8 as *const SymbolName,
            addr: &raw const RtsFlags as *mut RTS_FLAGS as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_breakpoint_io_action\0" as *const u8 as *const SymbolName,
            addr: &raw const rts_breakpoint_io_action as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_rts_stop_next_breakpoint\0" as *const u8 as *const SymbolName,
            addr: &raw const rts_stop_next_breakpoint as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_rts_stop_on_exception\0" as *const u8 as *const SymbolName,
            addr: &raw const rts_stop_on_exception as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_rts_enableStopNextBreakpointAll\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                rts_enableStopNextBreakpointAll as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_disableStopNextBreakpointAll\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                rts_disableStopNextBreakpointAll as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_enableStopNextBreakpoint\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                rts_enableStopNextBreakpoint as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_disableStopNextBreakpoint\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                rts_disableStopNextBreakpoint as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_enableStopAfterReturn\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                rts_enableStopAfterReturn as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_disableStopAfterReturn\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                rts_disableStopAfterReturn as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stopTimer\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stopTimer as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_n_capabilities\0" as *const u8 as *const SymbolName,
            addr: &raw const n_capabilities as *mut u32 as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_max_n_capabilities\0" as *const u8 as *const SymbolName,
            addr: &raw const max_n_capabilities as *mut u32 as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_enabled_capabilities\0" as *const u8 as *const SymbolName,
            addr: &raw const enabled_capabilities as *mut u32 as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_traceEventzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_traceEventzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_traceMarkerzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_traceMarkerzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_traceBinaryEventzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_traceBinaryEventzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_getThreadAllocationCounterzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_getThreadAllocationCounterzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_getOtherThreadAllocationCounterzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_getOtherThreadAllocationCounterzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_setThreadAllocationCounterzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_setThreadAllocationCounterzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_setOtherThreadAllocationCounterzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_setOtherThreadAllocationCounterzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_getMonotonicNSec\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgWord64>, *mut c_void>(Some(
                getMonotonicNSec as unsafe extern "C" fn() -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_lockFile\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord64, StgWord64, StgWord64, c_int) -> c_int>,
                *mut c_void,
            >(Some(
                lockFile as unsafe extern "C" fn(StgWord64, StgWord64, StgWord64, c_int) -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_unlockFile\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord64) -> c_int>, *mut c_void>(Some(
                unlockFile as unsafe extern "C" fn(StgWord64) -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_startProfTimer\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                startProfTimer as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stopProfTimer\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stopProfTimer as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_startHeapProfTimer\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                startHeapProfTimer as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stopHeapProfTimer\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                stopHeapProfTimer as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_requestTickyCounterSamples\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                requestTickyCounterSamples as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_setUserEra\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> ()>, *mut c_void>(Some(
                setUserEra as unsafe extern "C" fn(StgWord) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_incrementUserEra\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                incrementUserEra as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_getUserEra\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgWord>, *mut c_void>(Some(
                getUserEra as unsafe extern "C" fn() -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_requestHeapCensus\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                requestHeapCensus as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_atomic_inc\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgVolatilePtr, StgWord) -> StgWord>,
                *mut c_void,
            >(Some(
                atomic_inc as unsafe extern "C" fn(StgVolatilePtr, StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_atomic_dec\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgVolatilePtr, StgWord) -> StgWord>,
                *mut c_void,
            >(Some(
                atomic_dec as unsafe extern "C" fn(StgVolatilePtr, StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_spt_lookup\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut StgWord64) -> StgPtr>, *mut c_void>(
                Some(hs_spt_lookup as unsafe extern "C" fn(*mut StgWord64) -> StgPtr),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_spt_insert\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut StgWord64, *mut c_void) -> ()>,
                *mut c_void,
            >(Some(
                hs_spt_insert as unsafe extern "C" fn(*mut StgWord64, *mut c_void) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_spt_insert_stableptr\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut StgWord64, *mut StgStablePtr) -> ()>,
                *mut c_void,
            >(Some(
                hs_spt_insert_stableptr
                    as unsafe extern "C" fn(*mut StgWord64, *mut StgStablePtr) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_spt_remove\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut StgWord64) -> ()>, *mut c_void>(
                Some(hs_spt_remove as unsafe extern "C" fn(*mut StgWord64) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_spt_keys\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut StgPtr, c_int) -> c_int>, *mut c_void>(
                Some(hs_spt_keys as unsafe extern "C" fn(*mut StgPtr, c_int) -> c_int),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_spt_key_count\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> c_int>, *mut c_void>(Some(
                hs_spt_key_count as unsafe extern "C" fn() -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_cas\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgVolatilePtr, StgWord, StgWord) -> StgWord>,
                *mut c_void,
            >(Some(
                cas as unsafe extern "C" fn(StgVolatilePtr, StgWord, StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"__assertFail\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*const c_char, c_uint) -> !>, *mut c_void>(
                Some(_assertFail as unsafe extern "C" fn(*const c_char, c_uint) -> !),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_keepCAFs\0" as *const u8 as *const SymbolName,
            addr: &raw const keepCAFs as *mut bool as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_registerInfoProvList\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut IpeBufferListNode) -> ()>,
                *mut c_void,
            >(Some(
                registerInfoProvList as unsafe extern "C" fn(*mut IpeBufferListNode) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_lookupIPE\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*const StgInfoTable, *mut InfoProvEnt) -> bool>,
                *mut c_void,
            >(Some(
                lookupIPE as unsafe extern "C" fn(*const StgInfoTable, *mut InfoProvEnt) -> bool,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_sendCloneStackMessage\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut StgTSO, HsStablePtr) -> ()>,
                *mut c_void,
            >(Some(
                sendCloneStackMessage as unsafe extern "C" fn(*mut StgTSO, HsStablePtr) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_cloneStack\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Capability, *const StgStack) -> *mut StgStack>,
                *mut c_void,
            >(Some(
                cloneStack
                    as unsafe extern "C" fn(*mut Capability, *const StgStack) -> *mut StgStack,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_newPromptTagzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_newPromptTagzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_promptzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_promptzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_control0zh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_control0zh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_newArena\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> *mut Arena>, *mut c_void>(Some(
                newArena as unsafe extern "C" fn() -> *mut Arena,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_arenaAlloc\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*mut Arena, usize) -> *mut c_void>,
                *mut c_void,
            >(Some(
                arenaAlloc as unsafe extern "C" fn(*mut Arena, usize) -> *mut c_void,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_arenaFree\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(*mut Arena) -> ()>, *mut c_void>(Some(
                arenaFree as unsafe extern "C" fn(*mut Arena) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rts_clearMemory\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                rts_clearMemory as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_setKeepCAFs\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                setKeepCAFs as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rtsBadAlignmentBarf\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> !>, *mut c_void>(Some(
                rtsBadAlignmentBarf as unsafe extern "C" fn() -> !,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rtsOutOfBoundsAccess\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> !>, *mut c_void>(Some(
                rtsOutOfBoundsAccess as unsafe extern "C" fn() -> !,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rtsMemcpyRangeOverlap\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> !>, *mut c_void>(Some(
                rtsMemcpyRangeOverlap as unsafe extern "C" fn() -> !,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_castWord64ToDoublezh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_castWord64ToDoublezh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_castDoubleToWord64zh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_castDoubleToWord64zh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_castWord32ToFloatzh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_castWord32ToFloatzh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_stg_castFloatToWord32zh\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, *mut c_void>(Some(
                stg_castFloatToWord32zh as unsafe extern "C" fn() -> StgFunPtr,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_closure_sizeW_\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(*const StgClosure, *const StgInfoTable) -> c_uint>,
                *mut c_void,
            >(Some(
                closure_sizeW_
                    as unsafe extern "C" fn(*const StgClosure, *const StgInfoTable) -> c_uint,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_setIOManagerControlFd\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_uint, c_int) -> ()>, *mut c_void>(
                Some(setIOManagerControlFd as unsafe extern "C" fn(c_uint, c_int) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_setTimerManagerControlFd\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int) -> ()>, *mut c_void>(Some(
                setTimerManagerControlFd as unsafe extern "C" fn(c_int) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_setIOManagerWakeupFd\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int) -> ()>, *mut c_void>(Some(
                setIOManagerWakeupFd as unsafe extern "C" fn(c_int) -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_blockUserSignals\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                blockUserSignals as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_unblockUserSignals\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                unblockUserSignals as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_CHARLIKE_closure\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_CHARLIKE_closure as *mut [StgIntCharlikeClosure; 256]
                as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_INTLIKE_closure\0" as *const u8 as *const SymbolName,
            addr: &raw const stg_INTLIKE_closure as *mut [StgIntCharlikeClosure; 272]
                as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"___hscore_get_saved_termios\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int) -> *mut c_void>, *mut c_void>(
                Some(__hscore_get_saved_termios as unsafe extern "C" fn(c_int) -> *mut c_void),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"___hscore_set_saved_termios\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int, *mut c_void) -> ()>, *mut c_void>(
                Some(__hscore_set_saved_termios as unsafe extern "C" fn(c_int, *mut c_void) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_shutdownHaskellAndSignal\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(c_int, c_int) -> !>, *mut c_void>(Some(
                shutdownHaskellAndSignal as unsafe extern "C" fn(c_int, c_int) -> !,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_signal_handlers\0" as *const u8 as *const SymbolName,
            addr: &raw const signal_handlers as *mut *mut StgInt as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_stg_sig_install\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(c_int, c_int, *mut c_void) -> c_int>,
                *mut c_void,
            >(Some(
                stg_sig_install as unsafe extern "C" fn(c_int, c_int, *mut c_void) -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_rtsTimerSignal\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> c_int>, *mut c_void>(Some(
                rtsTimerSignal as unsafe extern "C" fn() -> c_int,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_nocldstop\0" as *const u8 as *const SymbolName,
            addr: &raw const nocldstop as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"___udivti3\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                __udivti3 as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"___umodti3\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                __umodti3 as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_prep_cif\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                ffi_prep_cif as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_call\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn() -> ()>, *mut c_void>(Some(
                ffi_call as unsafe extern "C" fn() -> (),
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_void\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_void as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_float\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_float as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_double\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_double as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_sint64\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_sint64 as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_uint64\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_uint64 as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_sint32\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_sint32 as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_uint32\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_uint32 as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_sint16\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_sint16 as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_uint16\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_uint16 as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_sint8\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_sint8 as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_uint8\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_uint8 as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_ffi_type_pointer\0" as *const u8 as *const SymbolName,
            addr: &raw const ffi_type_pointer as *mut [StgWord; 0] as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_add8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_add8 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_add16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_add16 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_add32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_add32 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_add64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_atomic_add64 as unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_sub8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_sub8 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_sub16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_sub16 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_sub32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_sub32 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_sub64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_atomic_sub64 as unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_and8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_and8 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_and16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_and16 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_and32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_and32 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_and64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_atomic_and64 as unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_nand8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_nand8 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_nand16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_nand16 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_nand32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_nand32 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_nand64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_atomic_nand64 as unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_or8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_or8 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_or16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_or16 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_or32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_or32 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_or64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_atomic_or64 as unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_xor8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_xor8 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_xor16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_xor16 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_xor32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_atomic_xor32 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomic_xor64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_atomic_xor64 as unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_cmpxchg8\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord, StgWord) -> StgWord>,
                *mut c_void,
            >(Some(
                hs_cmpxchg8 as unsafe extern "C" fn(StgWord, StgWord, StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_cmpxchg16\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord, StgWord) -> StgWord>,
                *mut c_void,
            >(Some(
                hs_cmpxchg16 as unsafe extern "C" fn(StgWord, StgWord, StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_cmpxchg32\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord, StgWord) -> StgWord>,
                *mut c_void,
            >(Some(
                hs_cmpxchg32 as unsafe extern "C" fn(StgWord, StgWord, StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_cmpxchg64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord64, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_cmpxchg64 as unsafe extern "C" fn(StgWord, StgWord64, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_xchg8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_xchg8 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_xchg16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_xchg16 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_xchg32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_xchg32 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_xchg64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_xchg64 as unsafe extern "C" fn(StgWord, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomicread8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_atomicread8 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomicread16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_atomicread16 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomicread32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_atomicread32 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomicread64\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord64>, *mut c_void>(
                Some(hs_atomicread64 as unsafe extern "C" fn(StgWord) -> StgWord64),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomicwrite8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> ()>, *mut c_void>(
                Some(hs_atomicwrite8 as unsafe extern "C" fn(StgWord, StgWord) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomicwrite16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> ()>, *mut c_void>(
                Some(hs_atomicwrite16 as unsafe extern "C" fn(StgWord, StgWord) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomicwrite32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> ()>, *mut c_void>(
                Some(hs_atomicwrite32 as unsafe extern "C" fn(StgWord, StgWord) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_atomicwrite64\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord64) -> ()>, *mut c_void>(
                Some(hs_atomicwrite64 as unsafe extern "C" fn(StgWord, StgWord64) -> ()),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_bitrev8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_bitrev8 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_bitrev16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord16) -> StgWord16>, *mut c_void>(
                Some(hs_bitrev16 as unsafe extern "C" fn(StgWord16) -> StgWord16),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_bitrev32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord32) -> StgWord32>, *mut c_void>(
                Some(hs_bitrev32 as unsafe extern "C" fn(StgWord32) -> StgWord32),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_bitrev64\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord64) -> StgWord64>, *mut c_void>(
                Some(hs_bitrev64 as unsafe extern "C" fn(StgWord64) -> StgWord64),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_bswap16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord16) -> StgWord16>, *mut c_void>(
                Some(hs_bswap16 as unsafe extern "C" fn(StgWord16) -> StgWord16),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_bswap32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord32) -> StgWord32>, *mut c_void>(
                Some(hs_bswap32 as unsafe extern "C" fn(StgWord32) -> StgWord32),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_bswap64\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord64) -> StgWord64>, *mut c_void>(
                Some(hs_bswap64 as unsafe extern "C" fn(StgWord64) -> StgWord64),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_clz8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_clz8 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_clz16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_clz16 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_clz32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_clz32 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_clz64\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord64) -> StgWord>, *mut c_void>(
                Some(hs_clz64 as unsafe extern "C" fn(StgWord64) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_ctz8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_ctz8 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_ctz16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_ctz16 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_ctz32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_ctz32 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_ctz64\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord64) -> StgWord>, *mut c_void>(
                Some(hs_ctz64 as unsafe extern "C" fn(StgWord64) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_mulIntMayOflo\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(W_, W_) -> W_>, *mut c_void>(Some(
                hs_mulIntMayOflo as unsafe extern "C" fn(W_, W_) -> W_,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pdep8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_pdep8 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pdep16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_pdep16 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pdep32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_pdep32 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pdep64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord64, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_pdep64 as unsafe extern "C" fn(StgWord64, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pext8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_pext8 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pext16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_pext16 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pext32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_pext32 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pext64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord64, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_pext64 as unsafe extern "C" fn(StgWord64, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pext8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_pext8 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pext16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_pext16 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pext32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord, StgWord) -> StgWord>, *mut c_void>(
                Some(hs_pext32 as unsafe extern "C" fn(StgWord, StgWord) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_pext64\0" as *const u8 as *const SymbolName,
            addr: transmute::<
                Option<unsafe extern "C" fn(StgWord64, StgWord64) -> StgWord64>,
                *mut c_void,
            >(Some(
                hs_pext64 as unsafe extern "C" fn(StgWord64, StgWord64) -> StgWord64,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_popcnt\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_popcnt as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_popcnt8\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_popcnt8 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_popcnt16\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_popcnt16 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_popcnt32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgWord>, *mut c_void>(Some(
                hs_popcnt32 as unsafe extern "C" fn(StgWord) -> StgWord,
            )),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_popcnt64\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord64) -> StgWord>, *mut c_void>(
                Some(hs_popcnt64 as unsafe extern "C" fn(StgWord64) -> StgWord),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_word2float32\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgFloat>, *mut c_void>(
                Some(hs_word2float32 as unsafe extern "C" fn(StgWord) -> StgFloat),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_hs_word2float64\0" as *const u8 as *const SymbolName,
            addr: transmute::<Option<unsafe extern "C" fn(StgWord) -> StgDouble>, *mut c_void>(
                Some(hs_word2float64 as unsafe extern "C" fn(StgWord) -> StgDouble),
            ),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
        _RtsSymbolVal {
            lbl: b"_nonmoving_write_barrier_enabled\0" as *const u8 as *const SymbolName,
            addr: &raw const nonmoving_write_barrier_enabled as *mut StgWord as *mut c_void,
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_DATA,
        },
        _RtsSymbolVal {
            lbl: null::<SymbolName>(),
            addr: null_mut::<c_void>(),
            strength: STRENGTH_NORMAL,
            r#type: SYM_TYPE_CODE,
        },
    ]
};

static mut default_extra_syms: [RtsSymbolVal; 1] = [_RtsSymbolVal {
    lbl: null::<SymbolName>(),
    addr: null_mut::<c_void>(),
    strength: STRENGTH_NORMAL,
    r#type: SYM_TYPE_CODE,
}];

unsafe fn rtsExtraSyms() -> *mut RtsSymbolVal {
    return &raw mut default_extra_syms as *mut RtsSymbolVal;
}
