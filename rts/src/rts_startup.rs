use crate::adjustor::initAdjustors;
use crate::builtin_closures::initBuiltinClosures;
use crate::capability::getCapability;
use crate::check_vector_support::setVectorSupport;
use crate::eventlog::event_log::{finishCapEventLogging, postInitEvent};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::hpc::{exitHpc, startupHpc};
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts::os_threads::freeThreadingResources;
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::stable_ptr::getStablePtr;
use crate::ffi::rts::storage::gc::{generations, setKeepCAFs};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::timer::{startTimer, stopTimer};
use crate::ffi::rts_api::{
    HaskellObj, RtsConfig, RtsOptsAll, defaultRtsConfig, rts_evalIO, rts_lock, rts_unlock,
    setFullProgArgv,
};
use crate::ffi::stg::smp::{atomic_dec, atomic_inc};
use crate::ffi::stg::types::{StgInt, StgPtr, StgVolatilePtr, StgWord};
use crate::file_lock::{freeFileLocking, initFileLocking};
use crate::foreign_exports::processForeignExports;
use crate::get_time::initializeTimer;
use crate::globals::{exitGlobalStore, initGlobalStore};
use crate::io_manager::{exitIOManager, initIOManager, selectIOManager, stopIOManager};
use crate::ipe::{dumpIPEToEventLog, exitIpe, initIpe};
use crate::libdw_pool::libdwPoolInit;
use crate::linker::m_map::initLinkerMMap;
use crate::linker_internals::exitLinker;
use crate::posix::tty::resetTerminalSettings;
use crate::prelude::*;
use crate::prof_heap::{endHeapProfiling, freeHeapProfiling, initHeapProfiling};
use crate::rts_flags::{freeRtsArgs, initRtsFlagsDefaults, rtsConfig, setupRtsFlags};
use crate::rts_signals::{
    freeSignalHandlers, initDefaultHandlers, initUserSignals, resetDefaultHandlers,
};
use crate::schedule::{exitScheduler, freeScheduler, initScheduler};
use crate::sm::non_moving_mark::nonmoving_weak_ptr_list;
use crate::sm::storage::{exitStorage, freeStorage, initStorage};
use crate::stable_name::{exitStableNameTable, initStableNameTable};
use crate::stable_ptr::{exitStablePtrTable, initStablePtrTable};
use crate::static_ptr_table::exitStaticPtrTable;
use crate::stats::{
    initStats0, initStats1, stat_endExit, stat_endInit, stat_exit, stat_startExit, stat_startInit,
};
use crate::timer::{exitTimer, initTimer};
use crate::top_handler::{exitTopHandler, initTopHandler};
use crate::trace::{
    endTracing, flushTrace, freeTracing, initTracing, traceOSProcessInfo, traceWallClockTime,
};
use crate::weak::runAllCFinalizers;

#[cfg(test)]
mod tests;

extern "C" {
    pub(crate) fn init_ghc_hs_iface();
}

static mut hs_init_count: StgWord = 0 as StgWord;

static mut rts_shutdown: bool = r#false != 0;

unsafe fn x86_init_fpu() {}

unsafe fn initBuiltinGcRoots() {
    getStablePtr((*ghc_hs_iface).runIO_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).runNonIO_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).flushStdHandles_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).runFinalizzerBatch_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).stackOverflow_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).heapOverflow_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).unpackCString_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).blockedIndefinitelyOnMVar_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).nonTermination_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).blockedIndefinitelyOnSTM_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).allocationLimitExceeded_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).cannotCompactFunction_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).cannotCompactPinned_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).cannotCompactMutable_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).nestedAtomically_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).underflowException_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).overflowException_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).divZZeroException_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).runSparks_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).ensureIOManagerIsRunning_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).interruptIOManager_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).ioManagerCapabilitiesChanged_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).runHandlersPtr_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).absentSumFieldError_closure as StgPtr);
    getStablePtr((*ghc_hs_iface).runAllocationLimitHandler_closure as StgPtr);
}

#[ffi(docs, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_init(mut argc: *mut c_int, mut argv: *mut *mut *mut c_char) {
    hs_init_ghc(argc, argv, defaultRtsConfig);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_init_with_rtsopts(
    mut argc: *mut c_int,
    mut argv: *mut *mut *mut c_char,
) {
    let mut rts_opts = defaultRtsConfig;
    rts_opts.rts_opts_enabled = RtsOptsAll;
    hs_init_ghc(argc, argv, rts_opts);
}

#[ffi(compiler, docs, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_init_ghc(
    mut argc: *mut c_int,
    mut argv: *mut *mut *mut c_char,
    mut rts_config: RtsConfig,
) {
    let mut init_count = atomic_inc(&raw mut hs_init_count as StgVolatilePtr, 1 as StgWord);

    if init_count > 1 as StgWord {
        return;
    }

    if rts_shutdown {
        errorBelch(
            b"hs_init_ghc: reinitializing the RTS after shutdown is not currently supported\0"
                as *const u8 as *const c_char,
        );

        stg_exit(1 as c_int);
    }

    setlocale(LC_CTYPE, b"\0" as *const u8 as *const c_char);
    init_ghc_hs_iface();
    initStats0();
    initializeTimer();
    stat_startInit();
    initRtsFlagsDefaults();
    rts_config.defaultsHook.expect("non-null function pointer")();

    if rts_config.keep_cafs != 0 {
        setKeepCAFs();
    }

    if argc.is_null() || argv.is_null() {
        let mut my_argc = 1 as c_int;

        let mut my_argv: [*mut c_char; 2] = [
            b"<unknown>\0" as *const u8 as *const c_char as *mut c_char,
            null_mut::<c_char>(),
        ];

        setFullProgArgv(my_argc, &raw mut my_argv as *mut *mut c_char);

        setupRtsFlags(
            &raw mut my_argc,
            &raw mut my_argv as *mut *mut c_char,
            rts_config,
        );
    } else {
        setFullProgArgv(*argc, *argv);
        setupRtsFlags(argc, *argv, rts_config);
    }

    selectIOManager();
    setVectorSupport();
    initAdjustors();
    initLinkerMMap();
    initStats1();
    initTracing();
    libdwPoolInit();
    initTimer();
    initScheduler();
    postInitEvent(Some(traceWallClockTime as unsafe extern "C" fn() -> ()));
    postInitEvent(Some(traceOSProcessInfo as unsafe extern "C" fn() -> ()));
    flushTrace();
    initBuiltinClosures();
    initStorage();
    initStablePtrTable();
    initStableNameTable();
    initBuiltinGcRoots();
    processForeignExports();
    initTopHandler();
    initGlobalStore();
    initFileLocking();
    initIpe();
    postInitEvent(Some(dumpIPEToEventLog as unsafe extern "C" fn() -> ()));
    initHeapProfiling();
    startTimer();

    if RtsFlags.MiscFlags.install_signal_handlers {
        initUserSignals();
        initDefaultHandlers();
    }

    initIOManager();
    x86_init_fpu();
    startupHpc();
    stat_endInit();
}

unsafe fn startupHaskell(
    mut argc: c_int,
    mut argv: *mut *mut c_char,
    mut init_root: Option<unsafe extern "C" fn() -> ()>,
) {
    hs_init(&raw mut argc, &raw mut argv);
}

unsafe fn hs_exit_(mut wait_foreign: bool) {
    let mut g: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut init_count: StgInt =
        atomic_dec(&raw mut hs_init_count as StgVolatilePtr, 1 as StgWord) as StgInt;

    if init_count > 0 as StgInt {
        return;
    }

    if init_count < 0 as StgInt {
        errorBelch(b"warning: too many hs_exit()s\0" as *const u8 as *const c_char);
        return;
    }

    rts_shutdown = r#true != 0;
    stat_startExit();
    rtsConfig.onExitHook.expect("non-null function pointer")();
    flushStdHandles();
    stopIOManager();
    exitScheduler(wait_foreign);
    i = 0 as uint32_t;

    while i < getNumCapabilities() as uint32_t {
        runAllCFinalizers((*getCapability(i)).weak_ptr_list_hd);
        i = i.wrapping_add(1);
    }

    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        runAllCFinalizers((*generations.offset(g as isize)).weak_ptr_list);
        g = g.wrapping_add(1);
    }

    runAllCFinalizers(nonmoving_weak_ptr_list);

    if RtsFlags.MiscFlags.install_signal_handlers {
        freeSignalHandlers();
    }

    stopTimer();
    exitTimer(r#true != 0);
    resetTerminalSettings();

    if RtsFlags.MiscFlags.install_signal_handlers {
        resetDefaultHandlers();
    }

    stat_endExit();
    exitHpc();
    exitStorage();
    finishCapEventLogging();
    freeScheduler();
    exitGlobalStore();
    exitLinker();
    freeFileLocking();
    exitStaticPtrTable();
    exitTopHandler();
    exitStablePtrTable();
    exitStableNameTable();
    endHeapProfiling();
    freeHeapProfiling();
    endTracing();
    freeTracing();
    exitIOManager(wait_foreign);
    stat_exit();
    freeStorage(wait_foreign);
    freeRtsArgs();
    freeThreadingResources();
    exitIpe();
}

unsafe fn flushStdHandles() {
    let mut cap = null_mut::<Capability>();
    cap = rts_lock();

    rts_evalIO(
        &raw mut cap,
        (*ghc_hs_iface).flushStdHandles_closure as HaskellObj,
        null_mut::<HaskellObj>(),
    );

    rts_unlock(cap);
}

#[ffi(docs, ghc_lib, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_exit() {
    hs_exit_(r#true != 0);
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_exit_nowait() {
    hs_exit_(r#false != 0);
}

unsafe fn shutdownHaskell() {
    hs_exit();
}

#[ffi(ghc_lib, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownHaskellAndExit(mut n: c_int, mut fastExit: c_int) -> ! {
    if fastExit == 0 {
        hs_exit_(r#false != 0);
    }

    stg_exit(n);
}

#[ffi(ghc_lib, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownHaskellAndSignal(mut sig: c_int, mut fastExit: c_int) -> ! {
    if fastExit == 0 {
        hs_exit_(r#false != 0);
    }

    exitBySignal(sig);
}

unsafe fn exitBySignal(mut sig: c_int) -> ! {
    let mut dfl = sigaction {
        __sigaction_u: __sigaction_u { __sa_handler: None },
        sa_mask: 0,
        sa_flags: 0,
    };

    let mut sigset: sigset_t = 0;
    dfl.sa_mask = 0 as sigset_t;
    dfl.sa_flags = 0 as c_int;
    dfl.__sigaction_u.__sa_handler = SIG_DFL;
    sigaction(sig, &raw mut dfl, null_mut::<sigaction>());
    sigset = 0 as sigset_t;
    sigset |= __sigbits(sig) as sigset_t;
    sigprocmask(SIG_UNBLOCK, &raw mut sigset, null_mut::<sigset_t>());

    match sig {
        SIGSTOP | SIGTSTP | SIGTTIN | SIGTTOU | SIGCONT => {
            exit(0xff as c_int);
        }
        _ => {
            kill(getpid(), sig);
            exit(0xff as c_int);
        }
    };
}

static mut exitFn: Option<unsafe extern "C" fn(c_int) -> ()> = None;

#[ffi(ghc_lib, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stg_exit(mut n: c_int) -> ! {
    if exitFn.is_some() {
        Some(exitFn.expect("non-null function pointer")).expect("non-null function pointer")(n);
    }

    exit(n);
}
