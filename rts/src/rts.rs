macro_rules! RTS_UNLIKELY {
    ($expr:expr) => {
        ::std::hint::unlikely($expr)
    };
}

pub(crate) use RTS_UNLIKELY;

macro_rules! RTS_LIKELY {
    ($expr:expr) => {
        ::std::hint::likely($expr)
    };
}

pub(crate) use RTS_LIKELY;
