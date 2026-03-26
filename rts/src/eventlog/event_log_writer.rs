use crate::ffi::rts::event_log_writer::EventLogWriter;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::sysErrorBelch;
use crate::ffi::rts::{prog_name, stg_exit};
use crate::ffi::stg::types::StgWord64;
use crate::fs::__rts_fopen;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

#[cfg(test)]
mod tests;

#[ffi(testsuite)]
#[repr(C)]
#[derive(Debug)]
pub struct EventLogWriter {
    pub initEventLogWriter: Option<unsafe extern "C" fn() -> ()>,
    pub writeEventLog: Option<unsafe extern "C" fn(*mut c_void, size_t) -> bool>,
    pub flushEventLog: Option<unsafe extern "C" fn() -> ()>,
    pub stopEventLogWriter: Option<unsafe extern "C" fn() -> ()>,
}

static mut event_log_pid: pid_t = -(1 as pid_t);

static mut event_log_file: *mut FILE = null::<FILE>() as *mut FILE;

unsafe fn acquire_event_log_lock() {}

unsafe fn release_event_log_lock() {}

unsafe fn outputFileName() -> *mut c_char {
    if !RtsFlags.TraceFlags.trace_output.is_null() {
        return strdup(RtsFlags.TraceFlags.trace_output);
    } else {
        let mut prog = stgMallocBytes(
            strlen(prog_name).wrapping_add(1 as size_t),
            b"initEventLogFileWriter\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut c_char;

        strcpy(prog, prog_name);

        let mut filename = stgMallocBytes(
            strlen(prog)
                .wrapping_add(10 as size_t)
                .wrapping_add(10 as size_t),
            b"initEventLogFileWriter\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut c_char;

        if event_log_pid == -(1 as pid_t) {
            sprintf(
                filename,
                b"%s.eventlog\0" as *const u8 as *const c_char,
                prog,
            );

            event_log_pid = getpid();
        } else {
            event_log_pid = getpid();

            sprintf(
                filename,
                b"%s.%llu.eventlog\0" as *const u8 as *const c_char,
                prog,
                event_log_pid as StgWord64,
            );
        }

        stgFree(prog as *mut c_void);

        return filename;
    };
}

unsafe fn initEventLogFileWriter() {
    let mut event_log_filename = outputFileName();
    event_log_file = __rts_fopen(event_log_filename, b"wb+\0" as *const u8 as *const c_char);

    if event_log_file.is_null() {
        sysErrorBelch(
            b"initEventLogFileWriter: can't open %s\0" as *const u8 as *const c_char,
            event_log_filename,
        );

        stg_exit(EXIT_FAILURE);
    }

    stgFree(event_log_filename as *mut c_void);
}

unsafe fn writeEventLogFile(mut eventlog: *mut c_void, mut eventlog_size: size_t) -> bool {
    let mut begin = eventlog as *mut c_uchar;
    let mut remain: size_t = eventlog_size;
    acquire_event_log_lock();

    while remain > 0 as size_t {
        let mut written =
            fwrite(begin as *const c_void, 1 as size_t, remain, event_log_file) as size_t;

        if written == 0 as size_t {
            release_event_log_lock();

            return r#false != 0;
        }

        remain = remain.wrapping_sub(written);
        begin = begin.offset(written as isize);
    }

    release_event_log_lock();
    flushEventLogFile();

    return r#true != 0;
}

unsafe fn flushEventLogFile() {
    if !event_log_file.is_null() {
        fflush(event_log_file);
    }
}

unsafe fn stopEventLogFileWriter() {
    if !event_log_file.is_null() {
        fclose(event_log_file);
        event_log_file = null_mut::<FILE>();
    }
}

unsafe fn initEventLogFileWriterNoop() {}

unsafe fn writeEventLogFileNoop(mut eventlog: *mut c_void, mut eventlog_size: size_t) -> bool {
    return r#true != 0;
}

unsafe fn flushEventLogFileNoop() {}

unsafe fn stopEventLogFileWriterNoop() {}

static mut FileEventLogWriter: EventLogWriter = unsafe {
    EventLogWriter {
        initEventLogWriter: Some(initEventLogFileWriter as unsafe extern "C" fn() -> ()),
        writeEventLog: Some(writeEventLogFile as unsafe extern "C" fn(*mut c_void, size_t) -> bool),
        flushEventLog: Some(flushEventLogFile as unsafe extern "C" fn() -> ()),
        stopEventLogWriter: Some(stopEventLogFileWriter as unsafe extern "C" fn() -> ()),
    }
};

static mut NullEventLogWriter: EventLogWriter = unsafe {
    EventLogWriter {
        initEventLogWriter: Some(initEventLogFileWriterNoop as unsafe extern "C" fn() -> ()),
        writeEventLog: Some(
            writeEventLogFileNoop as unsafe extern "C" fn(*mut c_void, size_t) -> bool,
        ),
        flushEventLog: Some(flushEventLogFileNoop as unsafe extern "C" fn() -> ()),
        stopEventLogWriter: Some(stopEventLogFileWriterNoop as unsafe extern "C" fn() -> ()),
    }
};
