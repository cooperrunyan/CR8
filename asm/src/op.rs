use crate::ast::Value;

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
            x => Err(format!("Invalid operation: {x:#?}")),
        }
    }
}

impl Operation {
    pub fn size(&self, _args: &Vec<Value>) -> Result<u8, String> {
        use Operation::*;
        let mut args = vec![];
        for arg in _args {
            args.push(arg)
        }

        macro_rules! none {
            ($a:expr, $n:expr) => {
                if $a.next().is_none() {
                    Ok($n as u8)
                } else {
                    dbg!(&$a);
                    Err("Too many arguments".to_string())
                }
            };
        }
        let mut args = args.into_iter();

        match self {
            LW | SW => {
                let mut args = match self {
                    LW => args.collect::<Vec<_>>().into_iter(),
                    SW => args.rev().collect::<Vec<_>>().into_iter(),
                    _ => panic!(),
                };

                let Some(Value::Register(_)) = args.next() else {
                    return Err("Expected the first argument of LW to be a register".to_string());
                };
                match args.next() {
                    None => return Ok(1),
                    Some(Value::AddrByte(_) | Value::Immediate(_)) => match args.next() {
                        Some(Value::AddrByte(_) | Value::Immediate(_)) => return none!(args, 3),
                        _ => return Err("Expected another address byte for LW".to_string()),
                    },
                    Some(Value::Expression(_) | Value::Ident(_)) => return none!(args, 3),
                    oth => return Err(format!("Unexpected additional argument: {oth:#?}")),
                }
            }
            PUSH => {
                match args.next() {
                    Some(Value::Immediate(_) | Value::Expression(_)) => return none!(args, 2),
                    Some(Value::Register(_)) => return none!(args, 1),
                    _ => return Err("Expected register or immediate as first argument".to_string()),
                };
            }
            JNZ => {
                let len = match args.next() {
                    Some(Value::Immediate(_) | Value::Expression(_)) => 2,
                    Some(Value::Register(_)) => 1,
                    _ => return Err("Expected register or immediate as first argument".to_string()),
                };
                return none!(args, len);
            }
            POP => {
                let Some(Value::Register(_)) = args.next() else {
                    return Err("Expected register as only argument".to_string());
                };
                return none!(args, 1);
            }
            OUT | IN | MOV | CMP | ADC | SBB | OR | NOR | AND => {
                let mut args = match self {
                    OUT => args.rev().collect::<Vec<_>>().into_iter(),
                    _ => args.collect::<Vec<_>>().into_iter(),
                };

                let Some(Value::Register(_)) = args.next() else {
                    return Err("Expected register argument".to_string());
                };
                match args.next() {
                    None => return Err("Expected another argument".to_string()),
                    Some(_) => return none!(args, 2),
                }
            }
        }
    }
}
