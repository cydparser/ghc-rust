use crate::ffi::rts::messages::debugBelch;
use crate::prelude::*;

unsafe fn reportMemoryMap() {
    debugBelch(b"\nMemory map:\n\0" as *const u8 as *const c_char);

    loop {
        let mut vmsize: mach_vm_size_t = 0;
        let mut address: mach_vm_address_t = 0;

        let mut info = vm_region_basic_info {
            protection: 0,
            max_protection: 0,
            inheritance: 0,
            shared: 0,
            reserved: 0,
            offset: 0,
            behavior: 0,
            user_wired_count: 0,
        };

        let mut flavor = VM_REGION_BASIC_INFO;
        let mut object: memory_object_name_t = 0;
        let mut info_count = VM_REGION_BASIC_INFO_COUNT;

        let mut kr = mach_vm_region(
            mach_task_self_ as vm_map_read_t,
            &raw mut address,
            &raw mut vmsize,
            flavor,
            &raw mut info as vm_region_info_t,
            &raw mut info_count,
            &raw mut object,
        );

        if kr == KERN_SUCCESS {
            debugBelch(
                b"%p-%p %8zuK %c%c%c/%c%c%c\n\0" as *const u8 as *const c_char,
                address as *mut c_void,
                address.wrapping_add(vmsize as mach_vm_address_t) as *mut c_void,
                vmsize as size_t >> 10 as c_int,
                if info.protection & VM_PROT_READ != 0 {
                    'r' as i32
                } else {
                    '-' as i32
                },
                if info.protection & VM_PROT_WRITE != 0 {
                    'w' as i32
                } else {
                    '-' as i32
                },
                if info.protection & VM_PROT_EXECUTE != 0 {
                    'x' as i32
                } else {
                    '-' as i32
                },
                if info.max_protection & VM_PROT_READ != 0 {
                    'r' as i32
                } else {
                    '-' as i32
                },
                if info.max_protection & VM_PROT_WRITE != 0 {
                    'w' as i32
                } else {
                    '-' as i32
                },
                if info.max_protection & VM_PROT_EXECUTE != 0 {
                    'x' as i32
                } else {
                    '-' as i32
                },
            );

            address = (address as uint64_t).wrapping_add(vmsize as uint64_t) as mach_vm_address_t
                as mach_vm_address_t;
        } else {
            if kr == KERN_INVALID_ADDRESS {
                break;
            }

            debugBelch(
                b"  Error: %s\n\0" as *const u8 as *const c_char,
                mach_error_string(kr as mach_error_t),
            );

            break;
        }
    }
}
