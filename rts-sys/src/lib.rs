#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![expect(unsafe_op_in_unsafe_fn)]

include!(concat!(env!("OUT_DIR"), "/rts.rs"));

/// Ensure doctests are linked correctly.
/// ```
/// use ghc_rts_sys::*;
/// assert_eq!(false, unsafe { keepCAFs });
/// unsafe { setKeepCAFs(); };
/// assert_eq!(true, unsafe { keepCAFs });
/// ````
const _: [(); 0] = [];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_links() {
        assert_eq!(false, unsafe { keepCAFs });
        unsafe {
            setKeepCAFs();
        };
        assert_eq!(true, unsafe { keepCAFs });
    }
}
