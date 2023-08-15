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

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "lw" => Self::LW,
            "sw" => Self::SW,
            "mov" => Self::MOV,
            "push" => Self::PUSH,
            "pop" => Self::POP,
            "jnz" => Self::JNZ,
            "inb" => Self::INB,
            "outb" => Self::OUTB,
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
