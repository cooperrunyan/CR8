macro_rules! impl_for {
    (for $for:ident, as $a:ty, impl $($t:ty),+) => {
        $(impl TryFrom<$t> for $for {
            type Error = String;
            fn try_from(value: $t) -> Result<Self, Self::Error> {
                Self::try_from(value as $a)
            }
        })*
    }
}

macro_rules! impl_str {
    (for $for:ident) => {
        impl TryFrom<String> for $for {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_from(value.as_str())
            }
        }
    };
}

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
    MB,
}

impl_for!(for Operation, as u64, impl u8, u16, u32, usize);

impl TryFrom<u64> for Operation {
    type Error = String;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::MOV),
            0x01 => Ok(Self::LW),
            0x02 => Ok(Self::SW),
            0x03 => Ok(Self::PUSH),
            0x04 => Ok(Self::POP),
            0x05 => Ok(Self::JNZ),
            0x06 => Ok(Self::IN),
            0x07 => Ok(Self::OUT),
            0x08 => Ok(Self::CMP),
            0x09 => Ok(Self::ADC),
            0x0A => Ok(Self::SBB),
            0x0B => Ok(Self::OR),
            0x0C => Ok(Self::NOR),
            0x0D => Ok(Self::AND),
            0x0E => Ok(Self::MB),

            x => Err(format!("Invalid operation: {x:#?}")),
        }
    }
}

impl_str!(for Operation);

impl TryFrom<&str> for Operation {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "mov" => Ok(Self::MOV),
            "lw" => Ok(Self::LW),
            "sw" => Ok(Self::SW),
            "push" => Ok(Self::PUSH),
            "pop" => Ok(Self::POP),
            "jnz" => Ok(Self::JNZ),
            "in" => Ok(Self::IN),
            "out" => Ok(Self::OUT),
            "cmp" => Ok(Self::CMP),
            "adc" => Ok(Self::ADC),
            "sbb" => Ok(Self::SBB),
            "or" => Ok(Self::OR),
            "nor" => Ok(Self::NOR),
            "and" => Ok(Self::AND),
            "mb" => Ok(Self::MB),
            x => Err(format!("Invalid operation: {x:#?}")),
        }
    }
}
