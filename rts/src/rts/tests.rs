use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_IN_STG_CODE() {
    assert_eq!(sys::IN_STG_CODE, IN_STG_CODE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq__REENTRANT() {
    assert_eq!(sys::_REENTRANT, _REENTRANT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FMT_SizeT() {
    assert_eq!(sys::FMT_SizeT, FMT_SizeT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FMT_HexSizeT() {
    assert_eq!(sys::FMT_HexSizeT, FMT_HexSizeT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_EXIT_INTERNAL_ERROR() {
    assert_eq!(sys::EXIT_INTERNAL_ERROR, EXIT_INTERNAL_ERROR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_EXIT_DEADLOCK() {
    assert_eq!(sys::EXIT_DEADLOCK, EXIT_DEADLOCK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_EXIT_INTERRUPTED() {
    assert_eq!(sys::EXIT_INTERRUPTED, EXIT_INTERRUPTED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_EXIT_HEAPOVERFLOW() {
    assert_eq!(sys::EXIT_HEAPOVERFLOW, EXIT_HEAPOVERFLOW);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_EXIT_KILLED() {
    assert_eq!(sys::EXIT_KILLED, EXIT_KILLED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_DEBUG_IS_ON() {
    assert_eq!(sys::DEBUG_IS_ON, DEBUG_IS_ON);
}

#[test]
#[ignore]
fn test_reportStackOverflow() {
    let tso = null_mut();
    unsafe { reportStackOverflow(tso) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_reportHeapOverflow() {
    unsafe { reportHeapOverflow() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stg_exit() {
    let n = Default::default();
    unsafe { stg_exit(n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_stg_sig_install(arg1: c_int, arg2: c_int) -> bool {
    let arg3 = null_mut();
    let expected = unsafe { sys::stg_sig_install(arg1, arg2, arg3) };
    let actual = unsafe { stg_sig_install(arg1, arg2, arg3) };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_sig_install() {
    let arg1 = Default::default();
    let arg2 = Default::default();
    let arg3 = null_mut();
    unsafe { stg_sig_install(arg1, arg2, arg3) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isProfiled() -> bool {
    let expected = unsafe { sys::rts_isProfiled() };
    let actual = unsafe { rts_isProfiled() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isProfiled() {
    unsafe { rts_isProfiled() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isDynamic() -> bool {
    let expected = unsafe { sys::rts_isDynamic() };
    let actual = unsafe { rts_isDynamic() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isDynamic() {
    unsafe { rts_isDynamic() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isThreaded() -> bool {
    let expected = unsafe { sys::rts_isThreaded() };
    let actual = unsafe { rts_isThreaded() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isThreaded() {
    unsafe { rts_isThreaded() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isDebugged() -> bool {
    let expected = unsafe { sys::rts_isDebugged() };
    let actual = unsafe { rts_isDebugged() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isDebugged() {
    unsafe { rts_isDebugged() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isTracing() -> bool {
    let expected = unsafe { sys::rts_isTracing() };
    let actual = unsafe { rts_isTracing() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isTracing() {
    unsafe { rts_isTracing() };
    todo!("assert")
}
