use anyhow::{bail, Result};
use std::fmt::Display;

use crate::compiler::lex::Value;

/// Native instructions
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    MOV = 0,
    JNZ = 1,
    LW = 2,
    SW = 3,
    PUSH = 4,
    POP = 5,
    IN = 6,
    OUT = 7,
    ADC = 8,
    SBB = 9,
    CMP = 10,
    AND = 11,
    OR = 12,
    NOR = 13,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperationArgAmt {
    R1I0 = 0b00,
    R1I1 = 0b01,
    R2I0 = 0b10,
    R0I1 = 0b11,
}

impl From<u8> for OperationArgAmt {
    fn from(value: u8) -> Self {
        let v = value & 0b11;
        match v {
            0b00 => Self::R1I0,
            0b01 => Self::R1I1,
            0b10 => Self::R2I0,
            0b11 => Self::R0I1,
            _ => panic!(),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operation as O;
        let str = match self {
            O::MOV => "mov",
            O::JNZ => "jnz",
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
            2 => O::LW,
            3 => O::SW,
            4 => O::PUSH,
            5 => O::POP,
            6 => O::IN,
            7 => O::OUT,
            8 => O::ADC,
            9 => O::SBB,
            10 => O::CMP,
            11 => O::AND,
            12 => O::OR,
            13 => O::NOR,
            _ => Err(())?,
        })
    }
}

impl Operation {
    pub fn size(&self, arg_amt: OperationArgAmt) -> Result<usize> {
        use Operation as O;
        use OperationArgAmt as AMT;
        let size = match self {
            O::LW | O::SW => match arg_amt {
                AMT::R1I1 => 4, // header + reg + imm + imm
                AMT::R1I0 => 2,
                _ => bail!("Invalid arg amounts for {self:#?}"),
            },
            O::MOV | O::IN | O::OUT | O::ADC | O::SBB | O::CMP | O::AND | O::OR | O::NOR => {
                match arg_amt {
                    AMT::R2I0 => 2,
                    AMT::R1I1 => 3,
                    _ => bail!("Invalid arg amounts for {self:#?}"),
                }
            }
            O::JNZ => match arg_amt {
                AMT::R1I0 => 2,
                AMT::R0I1 => 1, // If the "if" condition to jnz is known at compile time, effectively "jmp"
                _ => bail!("Invalid arg amounts for {self:#?}"),
            },
            O::POP => match arg_amt {
                AMT::R1I0 => 2,
                _ => bail!("Invalid arg amounts for {self:#?}"),
            },
            O::PUSH => match arg_amt {
                AMT::R1I0 | AMT::R0I1 => 2,
                _ => bail!("Invalid arg amounts for {self:#?}"),
            },
        };

        Ok(size)
    }
}

impl OperationArgAmt {
    pub fn from_args(args: &[Value]) -> Result<Self> {
        let mut regs = 0;
        let mut imms = 0;

        for arg in args {
            match arg {
                Value::Register(_) => regs += 1,
                _ => imms += 1,
            }
        }

        let op_arg_amt = match (regs, imms) {
            (1, 0) => OperationArgAmt::R1I0,
            (1, 1 | 2) => OperationArgAmt::R1I1,
            (2, 0) => OperationArgAmt::R2I0,
            (0, 1 | 2) => OperationArgAmt::R0I1,
            (a, b) => bail!("Unexpected registers {a:#?} + immediates {b:#?} combo"),
        };
        Ok(op_arg_amt)
    }
}
