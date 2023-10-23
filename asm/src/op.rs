use anyhow::{bail, Result};
use std::fmt::Display;

use crate::compiler::lex::Value;

/// Native instructions
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    MOV = 0b000000,
    JNZ = 0b000001,
    LW = 0b000010,
    SW = 0b000011,
    PUSH = 0b000100,
    POP = 0b000101,
    IN = 0b000110,
    OUT = 0b000111,
    ADC = 0b010000,
    SBB = 0b010001,
    CMP = 0b011000,
    NOT = 0b011001,
    AND = 0b011010,
    NAND = 0b011011,
    OR = 0b011100,
    NOR = 0b011101,
    XOR = 0b011110,
    XNOR = 0b011111,
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
            O::NOT => "not",
            O::AND => "and",
            O::NAND => "nand",
            O::OR => "or",
            O::NOR => "nor",
            O::XOR => "xor",
            O::XNOR => "xnor",
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
            "not" => O::NOT,
            "and" => O::AND,
            "nand" => O::NAND,
            "or" => O::OR,
            "nor" => O::NOR,
            "xor" => O::XOR,
            "xnor" => O::XNOR,
            _ => Err(())?,
        })
    }
}

impl TryFrom<u8> for Operation {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Operation as O;
        Ok(match value {
            0b000000 => O::MOV,
            0b000001 => O::JNZ,
            0b000010 => O::LW,
            0b000011 => O::SW,
            0b000100 => O::PUSH,
            0b000101 => O::POP,
            0b000110 => O::IN,
            0b000111 => O::OUT,
            0b010000 => O::ADC,
            0b010001 => O::SBB,
            0b011000 => O::CMP,
            0b011001 => O::NOT,
            0b011010 => O::AND,
            0b011011 => O::NAND,
            0b011100 => O::OR,
            0b011101 => O::NOR,
            0b011110 => O::XOR,
            0b011111 => O::XNOR,
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
            O::MOV
            | O::IN
            | O::OUT
            | O::ADC
            | O::SBB
            | O::CMP
            | O::AND
            | O::NAND
            | O::OR
            | O::NOR
            | O::XOR
            | O::XNOR => match arg_amt {
                AMT::R2I0 => 2,
                AMT::R1I1 => 3,
                _ => bail!("Invalid arg amounts for {self:#?}"),
            },
            O::JNZ => match arg_amt {
                AMT::R1I0 => 2,
                AMT::R0I1 => 1, // If the "if" condition to jnz is known at compile time, effectively "jmp"
                _ => bail!("Invalid arg amounts for {self:#?}"),
            },
            O::NOT | O::POP => match arg_amt {
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
