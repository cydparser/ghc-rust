use std::ops::{BitOr, BitOrAssign};

macro_rules! bitset_enum {
    ($vis:vis enum $name:ident { $($variant:ident = $disc:literal,)+ }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis enum $name {
            $($variant = $disc,)+
        }

        impl $name {
            pub const fn variants() -> &'static [$name] {
                &[
                    $($name::$variant),+
                ]
            }

            const fn mask() -> u32 {
                $($name::$variant as u32)|+
            }
        }
    }
}

#[rustfmt::skip]
bitset_enum! {
    pub enum Consumer {
        Compiler  = 0b0000001,
        Docs      = 0b0000010,
        Driver    = 0b0000100,
        GhcLib    = 0b0001000,
        Libraries = 0b0010000,
        Testsuite = 0b0100000,
        Utils     = 0b1000000,
    }
}

impl Consumer {
    pub fn to_str(self) -> &'static str {
        match self {
            Consumer::Compiler => "compiler",
            Consumer::Docs => "docs",
            Consumer::Driver => "driver",
            Consumer::GhcLib => "ghc_lib",
            Consumer::Libraries => "libraries",
            Consumer::Testsuite => "testsuite",
            Consumer::Utils => "utils",
        }
    }
}

/// A set of API consumers.
#[derive(Clone, Copy, Default)]
pub struct Consumers(u32);

impl Consumers {
    pub fn new() -> Self {
        Consumers::default()
    }

    pub fn insert(&mut self, consumer: Consumer) {
        self.0 |= consumer as u32;
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Debug for Consumers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#0b}", self.0)
    }
}

impl TryFrom<u32> for Consumers {
    type Error = ();

    fn try_from(bitset: u32) -> Result<Self, ()> {
        if bitset & !Consumer::mask() == 0 {
            Ok(Consumers(bitset))
        } else {
            Err(())
        }
    }
}

impl PartialEq<Consumer> for Consumers {
    fn eq(&self, consumer: &Consumer) -> bool {
        self.0 == *consumer as u32
    }
}

impl BitOr for Consumers {
    type Output = Consumers;

    fn bitor(self, rhs: Self) -> Self {
        Consumers(self.0 | rhs.0)
    }
}

impl BitOrAssign for Consumers {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

pub struct ConsumerIter {
    conusmers: u32,
    index: usize,
}

impl Iterator for ConsumerIter {
    type Item = Consumer;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(consumer) = Consumer::variants().get(self.index) {
            self.index += 1;

            if ((*consumer as u32) & self.conusmers) != 0 {
                return Some(*consumer);
            }
        }
        None
    }
}

impl IntoIterator for Consumers {
    type Item = Consumer;

    type IntoIter = ConsumerIter;

    fn into_iter(self) -> Self::IntoIter {
        ConsumerIter {
            conusmers: self.0,
            index: 0,
        }
    }
}
