/// Calls _sys::_ function when _sys_ feature is enabled. Emits `unimplemented` otherwise. This is
/// only intended to be used by generated code.
macro_rules! sys {
    (@ $fn:ident $($expr:tt)*) => {
        #[cfg(feature = "sys")]
        unsafe {
            #[allow(clippy::missing_transmute_annotations)]
            $($expr)*
        }
        #[cfg(not(feature = "sys"))]
        unimplemented!(stringify!($fn))
    };
    (transmute($fn:ident $($rest:tt)*)) => {
        sys! {
            @ $fn transmute(sys::$fn $($rest)*)
        }
    };
    ($fn:ident $($rest:tt)*) => {
        sys! {
            @ $fn sys::$fn $($rest)*
        }
    };
}

pub(crate) use sys;
