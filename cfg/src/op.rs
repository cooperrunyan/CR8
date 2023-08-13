#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    LW,
    SW,
    MOV,
    PUSH,
    POP,
    JNZ,
    INB,
    OUTB,
    CMP,
    ADC,
    SBB,
    OR,
    NOR,
    AND,
}

impl From<u8> for Operation {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::LW,
            0x01 => Self::SW,
            0x02 => Self::MOV,
            0x03 => Self::PUSH,
            0x04 => Self::POP,
            0x05 => Self::JNZ,
            0x06 => Self::INB,
            0x07 => Self::OUTB,
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

impl Into<u8> for Operation {
    fn into(self) -> u8 {
        match self {
            Self::LW => 0x00,
            Self::SW => 0x01,
            Self::MOV => 0x02,
            Self::PUSH => 0x03,
            Self::POP => 0x04,
            Self::JNZ => 0x05,
            Self::INB => 0x06,
            Self::OUTB => 0x07,
            Self::CMP => 0x08,
            Self::ADC => 0x09,
            Self::SBB => 0x0A,
            Self::OR => 0x0B,
            Self::NOR => 0x0C,
            Self::AND => 0x0D,
        }
    }
}
