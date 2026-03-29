use crate::prelude::*;

unsafe fn totalArgumentSize(mut typeString: *const c_char) -> i32 {
    let mut sz = 0;

    while *typeString != 0 {
        let fresh0 = typeString;
        typeString = typeString.offset(1);

        let mut t = *fresh0;
        let mut current_block_1: u64;

        match t as i32 {
            100 | 108 | 76 => {
                if size_of::<*mut c_void>() as usize == 4 {
                    sz += 2;
                    current_block_1 = 4906268039856690917;
                } else {
                    current_block_1 = 12333325718155969978;
                }
            }
            _ => {
                current_block_1 = 12333325718155969978;
            }
        }

        match current_block_1 {
            12333325718155969978 => {
                sz += 1;
            }
            _ => {}
        }
    }

    return sz;
}
