use crate::prelude::*;

pub(crate) static mut eventTypes: [EventType; 213] = [_EventType {
    etNum: 0,
    size: 0,
    desc: null_mut::<c_char>(),
}; 213];
