#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![expect(unsafe_op_in_unsafe_fn)]

include!(concat!(env!("OUT_DIR"), "/rts.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_links() {
        assert_eq!(false, unsafe { super::keepCAFs });
        unsafe {
            super::setKeepCAFs();
        };
        assert_eq!(true, unsafe { super::keepCAFs });
    }
}
