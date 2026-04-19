use crate::adjustor::initAdjustors;
use crate::builtin_closures::initBuiltinClosures;
use crate::capability::getCapability;
use crate::check_vector_support::setVectorSupport;
use crate::eventlog::event_log::{finishCapEventLogging, postInitEvent};
use crate::ffi::rts::hpc::{exitHpc, startupHpc};
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts::os_threads::freeThreadingResources;
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::stable_ptr::getStablePtr;
use crate::ffi::rts::storage::gc::{generations, setKeepCAFs};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::timer::{startTimer, stopTimer};
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
use crate::printer::DEBUG_LoadSymbols;
use crate::prof_heap::{endHeapProfiling, freeHeapProfiling, initHeapProfiling};
use crate::profiling::{endProfiling, freeProfiling, initProfiling, prof_file, reportCCSProfiling};
use crate::rts_api::{
    HaskellObj, RtsConfig, RtsOptsAll, defaultRtsConfig, rts_evalIO, rts_lock, rts_unlock,
    setFullProgArgv,
};
use crate::rts_flags::RtsFlags;
use crate::rts_flags::{freeRtsArgs, initRtsFlagsDefaults, rtsConfig, setupRtsFlags};
use crate::rts_signals::{
    freeSignalHandlers, initDefaultHandlers, initUserSignals, resetDefaultHandlers,
};
use crate::rts_utils::checkFPUStack;
use crate::schedule::{exitScheduler, freeScheduler, initScheduler};
use crate::sm::non_moving_mark::nonmoving_weak_ptr_list;
use crate::sm::storage::{exitStorage, freeStorage, initStorage};
use crate::stable_name::{exitStableNameTable, initStableNameTable};
use crate::stable_ptr::{exitStablePtrTable, initStablePtrTable};
use crate::static_ptr_table::exitStaticPtrTable;
use crate::stats::{
    initStats0, initStats1, stat_endExit, stat_endInit, stat_exit, stat_startExit, stat_startInit,
};
use crate::ticky::{PrintTickyInfo, emitTickyCounterDefs};
use crate::timer::{exitTimer, initTimer};
use crate::top_handler::{exitTopHandler, initTopHandler};
use crate::trace::{
    endTracing, flushTrace, freeTracing, initTracing, traceOSProcessInfo, traceWallClockTime,
};
use crate::weak::runAllCFinalizers;
use std::process;

#[cfg(test)]
mod tests;

extern "C" {
    pub(crate) fn init_ghc_hs_iface();
}

static mut hs_init_count: StgWord = 0;

static mut rts_shutdown: bool = false;

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
    let mut init_count = atomic_inc(&raw mut hs_init_count as StgVolatilePtr, 1);

    if init_count > 1 {
        return;
    }

    if rts_shutdown {
        errorBelch(
            c"hs_init_ghc: reinitializing the RTS after shutdown is not currently supported"
                .as_ptr(),
        );

        stg_exit(1);
    }

    libc::setlocale(libc::LC_CTYPE, c"".as_ptr());
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
        let mut my_argc = 1;

        let mut my_argv: [*const c_char; 2] = [c"<unknown>".as_ptr(), null_mut::<c_char>()];

        setFullProgArgv(my_argc, &raw mut my_argv as *mut *mut c_char);

        setupRtsFlags(
            &raw mut my_argc,
            &raw mut my_argv as *mut *mut c_char,
            rts_config,
        );
    } else {
        setFullProgArgv(*argc, *argv);
        setupRtsFlags(argc, *argv, rts_config);
        DEBUG_LoadSymbols(*(*argv).offset(0));
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
    initProfiling();
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
    mut argc: i32,
    mut argv: *mut *mut c_char,
    _init_root: Option<unsafe extern "C" fn() -> ()>,
) {
    hs_init(&raw mut argc, &raw mut argv);
}

unsafe fn hs_exit_(mut wait_foreign: bool) {
    let mut g: u32 = 0;
    let mut i: u32 = 0;
    let mut init_count: StgInt = atomic_dec(&raw mut hs_init_count as StgVolatilePtr, 1) as StgInt;

    if init_count > 0 {
        return;
    }

    if init_count < 0 {
        errorBelch(c"warning: too many hs_exit()s".as_ptr());
        return;
    }

    rts_shutdown = true;
    stat_startExit();
    rtsConfig.onExitHook.expect("non-null function pointer")();
    flushStdHandles();
    checkFPUStack();
    stopIOManager();
    exitScheduler(wait_foreign);
    i = 0;

    while i < getNumCapabilities() as u32 {
        runAllCFinalizers((*getCapability(i)).weak_ptr_list_hd);
        i = i.wrapping_add(1);
    }

    g = 0;

    while g < RtsFlags.GcFlags.generations {
        runAllCFinalizers((*generations.offset(g as isize)).weak_ptr_list);
        g = g.wrapping_add(1);
    }

    runAllCFinalizers(nonmoving_weak_ptr_list);

    if RtsFlags.MiscFlags.install_signal_handlers {
        freeSignalHandlers();
    }

    stopTimer();
    exitTimer(true);

    if RtsFlags.TraceFlags.ticky {
        emitTickyCounterDefs();
    }

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
    reportCCSProfiling();
    endHeapProfiling();
    freeHeapProfiling();
    endProfiling();
    freeProfiling();

    if !prof_file.is_null() {
        fclose(prof_file);
    }

    endTracing();
    freeTracing();

    if RtsFlags.TickyFlags.showTickyStats {
        PrintTickyInfo();
    }

    let mut tf = RtsFlags.TickyFlags.tickyFile;

    if !tf.is_null() {
        fclose(tf);
    }

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
    hs_exit_(true);
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_exit_nowait() {
    hs_exit_(false);
}

unsafe fn shutdownHaskell() {
    hs_exit();
}

#[ffi(ghc_lib, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownHaskellAndExit(mut n: c_int, mut fastExit: c_int) -> ! {
    if fastExit == 0 {
        hs_exit_(false);
    }

    stg_exit(n);
}

#[ffi(ghc_lib, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownHaskellAndSignal(mut sig: c_int, mut fastExit: c_int) -> ! {
    if fastExit == 0 {
        hs_exit_(false);
    }

    exitBySignal(sig);
}

unsafe fn exitBySignal(sig: i32) -> ! {
    // TODO(rust): Remove libc use.
    use libc::{
        SIG_DFL, SIG_UNBLOCK, SIGCONT, SIGSTOP, SIGTSTP, SIGTTIN, SIGTTOU, getpid, kill, sigaction,
        sigprocmask, sigset_t,
    };

    let mut dfl = libc::sigaction {
        sa_sigaction: SIG_DFL,
        sa_mask: 0,
        sa_flags: 0,
    };

    let mut sigset: sigset_t = 0;
    sigaction(sig, &raw mut dfl, null_mut::<sigaction>());
    sigset = 0;
    sigset |= __sigbits(sig) as sigset_t;
    sigprocmask(SIG_UNBLOCK, &raw mut sigset, null_mut::<sigset_t>());

    match sig {
        SIGSTOP | SIGTSTP | SIGTTIN | SIGTTOU | SIGCONT => {
            exit(0xff);
        }
        _ => {
            kill(getpid(), sig);
            exit(0xff);
        }
    };
}

static mut exitFn: Option<extern "C" fn(c_int) -> ()> = None;

#[ffi(ghc_lib, utils)]
#[unsafe(no_mangle)]
pub extern "C" fn stg_exit(n: c_int) -> ! {
    if let Some(exit_fn) = unsafe { exitFn } {
        exit_fn(n);
    }

    process::exit(n)
}
