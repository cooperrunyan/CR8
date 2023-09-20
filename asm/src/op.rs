use std::fmt::Display;

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

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operation as O;
        let str = match self {
            O::MOV => "mov",
            O::LW => "lw",
            O::SW => "sw",
            O::PUSH => "push",
            O::POP => "pop",
            O::JNZ => "jnz",
            O::IN => "in",
            O::OUT => "out",
            O::CMP => "cmp",
            O::ADC => "adc",
            O::SBB => "sbb",
            O::OR => "or",
            O::NOR => "nor",
            O::AND => "and",
            O::MB => "mb",
        };
        f.write_str(str)
    }
}

impl TryFrom<&str> for Operation {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Operation as O;
        Ok(match value {
            "mov" => O::MOV,
            "lw" => O::LW,
            "sw" => O::SW,
            "push" => O::PUSH,
            "pop" => O::POP,
            "jnz" => O::JNZ,
            "in" => O::IN,
            "out" => O::OUT,
            "cmp" => O::CMP,
            "adc" => O::ADC,
            "sbb" => O::SBB,
            "or" => O::OR,
            "nor" => O::NOR,
            "and" => O::AND,
            "mb" => O::MB,
            _ => Err(())?,
        })
    }
}

impl TryFrom<u8> for Operation {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Operation as O;
        Ok(match value {
            0x00 => O::MOV,
            0x01 => O::LW,
            0x02 => O::SW,
            0x03 => O::PUSH,
            0x04 => O::POP,
            0x05 => O::JNZ,
            0x06 => O::IN,
            0x07 => O::OUT,
            0x08 => O::CMP,
            0x09 => O::ADC,
            0x0A => O::SBB,
            0x0B => O::OR,
            0x0C => O::NOR,
            0x0D => O::AND,
            0x0E => O::MB,
            _ => Err(())?,
        })
    }
}
