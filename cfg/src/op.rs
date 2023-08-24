#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Operation {
    MOV,
    LW,
    SW,
    PUSH,
    POP,
    JNZ,
    IN,
    OUT,
    CMP,
    ADC,
    SBB,
    OR,
    NOR,
    AND,
}

impl From<u64> for Operation {
    fn from(value: u64) -> Self {
        match value {
            0x00 => Self::MOV,
            0x01 => Self::LW,
            0x02 => Self::SW,
            0x03 => Self::PUSH,
            0x04 => Self::POP,
            0x05 => Self::JNZ,
            0x06 => Self::IN,
            0x07 => Self::OUT,
            0x08 => Self::CMP,
            0x09 => Self::ADC,
            0x0A => Self::SBB,
            0x0B => Self::OR,
            0x0C => Self::NOR,
            0x0D => Self::AND,

            _ => panic!(),
        }
    }
}

macro_rules! uint {
    ($trait:ident) => {
        impl_uint!($trait, u8);
        impl_uint!($trait, u16);
        impl_uint!($trait, u32);
        impl_uint!($trait, usize);
    };
}

macro_rules! impl_uint {
    ($trait:ident, $t:ty) => {
        impl From<$t> for $trait {
            fn from(value: $t) -> Self {
                Self::from(value as u64)
            }
        }
    };
}

uint!(Operation);

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "mov" => Self::MOV,
            "lw" => Self::LW,
            "sw" => Self::SW,
            "push" => Self::PUSH,
            "pop" => Self::POP,
            "jnz" => Self::JNZ,
            "in" => Self::IN,
            "out" => Self::OUT,
            "cmp" => Self::CMP,
            "adc" => Self::ADC,
            "sbb" => Self::SBB,
            "or" => Self::OR,
            "nor" => Self::NOR,
            "and" => Self::AND,

            x => panic!("Invalid instruction name: {x}"),
        }
    }
}
