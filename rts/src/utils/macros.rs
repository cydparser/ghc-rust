/// Calls _sys::_ function when _sys_ feature is enabled. Emits `unimplemented` otherwise.
macro_rules! sys {
    ($fn:ident $($rest:tt)*) => {
        #[cfg(feature = "sys")]
        unsafe {
            sys::$fn$($rest)*
        }
        #[cfg(not(feature = "sys"))]
        unimplemented!(stringify!($fn))
    };
}

pub(crate) use sys;
