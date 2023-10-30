use anyhow::{anyhow, bail, Result};
use std::fmt::Display;

use crate::compiler::lex::Value;
use crate::compiler::Compiler;

/// Native instructions
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Operation {
    MOV,
    JNZ,
    JMP,
    LW,
    SW,
    PUSH,
    POP,
    IN,
    OUT,
    ADC,
    SBB,
    CMP,
    AND,
    OR,
    NOR,
    BANK,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operation as O;
        let str = match self {
            O::MOV => "mov",
            O::JNZ => "jnz",
            O::JMP => "jmp",
            O::LW => "lw",
            O::SW => "sw",
            O::PUSH => "push",
            O::POP => "pop",
            O::IN => "in",
            O::OUT => "out",
            O::ADC => "adc",
            O::SBB => "sbb",
            O::CMP => "cmp",
            O::AND => "and",
            O::OR => "or",
            O::NOR => "nor",
            O::BANK => "bank",
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
            "jnz" => O::JNZ,
            "jmp" => O::JMP,
            "lw" => O::LW,
            "sw" => O::SW,
            "push" => O::PUSH,
            "pop" => O::POP,
            "in" => O::IN,
            "out" => O::OUT,
            "adc" => O::ADC,
            "sbb" => O::SBB,
            "cmp" => O::CMP,
            "and" => O::AND,
            "or" => O::OR,
            "nor" => O::NOR,
            "bank" => O::BANK,
            _ => Err(())?,
        })
    }
}

impl TryFrom<u8> for Operation {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Operation as O;
        Ok(match value {
            0 => O::MOV,
            1 => O::JNZ,
            2 => O::JMP,
            3 => O::LW,
            4 => O::SW,
            5 => O::PUSH,
            6 => O::POP,
            7 => O::IN,
            8 => O::OUT,
            9 => O::ADC,
            10 => O::SBB,
            11 => O::CMP,
            12 => O::AND,
            13 => O::OR,
            14 => O::NOR,
            15 => O::BANK,
            _ => Err(())?,
        })
    }
}

impl Operation {
    pub fn compile(&self, args: Vec<Value>, ctx: &Compiler) -> Result<Vec<u8>> {
        let (_, is_imm) = self.check(&args)?;

        let mut reg_amt = 0;
        let mut reg_bits = 0b000;
        let mut bytes = vec![0];

        for arg in args {
            match arg {
                Value::Expr(e) => {
                    let val = e.resolve(ctx)?;
                    bytes.push(val as u8);
                    if matches!(self, Self::LW | Self::SW | Self::JNZ | Self::JMP) {
                        bytes.push((val >> 8) as u8);
                    }
                }
                Value::Literal(imm) => bytes.push(imm as u8),
                Value::Register(r) => {
                    if reg_amt > 0 {
                        bytes.push(r as u8);
                    } else {
                        reg_bits = (r as u8) & 0b111;
                        reg_amt += 1;
                    }
                }
                _ => bail!("Unexpected macro variable"),
            }
        }

        bytes[0] |= (*self as u8) << 4;
        bytes[0] |= reg_bits;
        if is_imm {
            bytes[0] |= 0b1000;
        }
        Ok(bytes)
    }

    pub fn check(&self, args: &[Value]) -> Result<(usize, bool)> {
        match self {
            Self::MOV => match args {
                [Value::Register(..), Value::Literal(..) | Value::Expr(..)] => Ok((2, true)),
                [Value::Register(..), Value::Register(..)] => Ok((2, false)),
                _ => Err(()),
            },
            Self::JNZ => match args {
                [Value::Literal(..), Value::Literal(..), Value::Register(..)] => Ok((3, true)),
                [Value::Expr(..), Value::Register(..)] => Ok((3, true)),
                [Value::Register(..)] => Ok((1, false)),
                _ => Err(()),
            },
            Self::JMP => match args {
                [Value::Literal(..), Value::Literal(..)] => Ok((3, true)),
                [Value::Expr(..)] => Ok((3, true)),
                [] => Ok((1, false)),
                _ => Err(()),
            },
            Self::LW => match args {
                [Value::Register(..), Value::Literal(..), Value::Literal(..)] => Ok((3, true)),
                [Value::Register(..), Value::Expr(..)] => Ok((3, true)),
                [Value::Register(..)] => Ok((1, false)),
                _ => Err(()),
            },
            Self::SW => match args {
                [Value::Literal(..), Value::Literal(..), Value::Register(..)] => Ok((3, true)),
                [Value::Expr(..), Value::Register(..)] => Ok((3, true)),
                [Value::Register(..)] => Ok((1, false)),
                _ => Err(()),
            },
            Self::PUSH => match args {
                [Value::Register(..)] => Ok((1, false)),
                [Value::Expr(..) | Value::Literal(..)] => Ok((2, true)),
                _ => Err(()),
            },
            Self::POP => match args {
                [Value::Register(..)] => Ok((1, false)),
                _ => Err(()),
            },
            Self::IN => match args {
                [Value::Register(..), Value::Register(..)] => Ok((2, false)),
                [Value::Register(..), Value::Literal(..) | Value::Expr(..)] => Ok((2, true)),
                _ => Err(()),
            },
            Self::OUT => match args {
                [Value::Register(..), Value::Register(..)] => Ok((2, false)),
                [Value::Literal(..) | Value::Expr(..), Value::Register(..)] => Ok((2, true)),
                _ => Err(()),
            },
            Self::ADC => match args {
                [Value::Register(..), Value::Register(..)] => Ok((2, false)),
                [Value::Register(..), Value::Literal(..) | Value::Expr(..)] => Ok((2, true)),
                _ => Err(()),
            },
            Self::SBB => match args {
                [Value::Register(..), Value::Register(..)] => Ok((2, false)),
                [Value::Register(..), Value::Literal(..) | Value::Expr(..)] => Ok((2, true)),
                _ => Err(()),
            },
            Self::CMP => match args {
                [Value::Register(..), Value::Register(..)] => Ok((2, false)),
                [Value::Register(..), Value::Literal(..) | Value::Expr(..)] => Ok((2, true)),
                _ => Err(()),
            },
            Self::AND => match args {
                [Value::Register(..), Value::Register(..)] => Ok((2, false)),
                [Value::Register(..), Value::Literal(..) | Value::Expr(..)] => Ok((2, true)),
                _ => Err(()),
            },
            Self::OR => match args {
                [Value::Register(..), Value::Register(..)] => Ok((2, false)),
                [Value::Register(..), Value::Literal(..) | Value::Expr(..)] => Ok((2, true)),
                _ => Err(()),
            },
            Self::NOR => match args {
                [Value::Register(..), Value::Register(..)] => Ok((2, false)),
                [Value::Register(..), Value::Literal(..) | Value::Expr(..)] => Ok((2, true)),
                _ => Err(()),
            },
            Self::BANK => match args {
                [Value::Register(..)] => Ok((1, false)),
                [Value::Literal(..) | Value::Expr(..)] => Ok((2, true)),
                _ => Err(()),
            },
        }
        .map_err(|_| anyhow!("Operation {self:#?} received invalid arg types"))
    }
}
