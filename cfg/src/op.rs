use std::{collections::HashMap, vec};

use crate::mem::Size;

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

#[derive(Debug, PartialEq, Eq)]
pub enum OpArg {
    Register,
    Immediate(Size),
    None,
}

lazy_static! {
    pub static ref NATIVE: HashMap<String, Vec<(OpArg, OpArg)>> = vec![
        (
            "lw".to_string(),
            vec![
                (OpArg::Register, OpArg::None),
                (OpArg::Register, OpArg::Immediate(Size::Word))
            ]
        ),
        (
            "sw".to_string(),
            vec![
                (OpArg::Register, OpArg::None),
                (OpArg::Immediate(Size::Word), OpArg::Register)
            ]
        ),
        (
            "mov".to_string(),
            vec![
                (OpArg::Register, OpArg::Register),
                (OpArg::Register, OpArg::Immediate(Size::Byte))
            ]
        ),
        (
            "push".to_string(),
            vec![
                (OpArg::Register, OpArg::None),
                (OpArg::Immediate(Size::Byte), OpArg::None)
            ]
        ),
        ("pop".to_string(), vec![(OpArg::Register, OpArg::None),]),
        (
            "jnz".to_string(),
            vec![
                (OpArg::Register, OpArg::None),
                (OpArg::Immediate(Size::Byte), OpArg::None)
            ]
        ),
        (
            "inb".to_string(),
            vec![
                (OpArg::Register, OpArg::Register),
                (OpArg::Register, OpArg::Immediate(Size::Byte))
            ]
        ),
        (
            "outb".to_string(),
            vec![
                (OpArg::Register, OpArg::Register),
                (OpArg::Immediate(Size::Byte), OpArg::Register),
            ]
        ),
        (
            "cmp".to_string(),
            vec![
                (OpArg::Register, OpArg::Register),
                (OpArg::Register, OpArg::Immediate(Size::Byte))
            ]
        ),
        (
            "adc".to_string(),
            vec![
                (OpArg::Register, OpArg::Register),
                (OpArg::Register, OpArg::Immediate(Size::Byte))
            ]
        ),
        (
            "sbb".to_string(),
            vec![
                (OpArg::Register, OpArg::Register),
                (OpArg::Register, OpArg::Immediate(Size::Byte))
            ]
        ),
        (
            "or".to_string(),
            vec![
                (OpArg::Register, OpArg::Register),
                (OpArg::Register, OpArg::Immediate(Size::Byte))
            ]
        ),
        (
            "nor".to_string(),
            vec![
                (OpArg::Register, OpArg::Register),
                (OpArg::Register, OpArg::Immediate(Size::Byte))
            ]
        ),
        (
            "and".to_string(),
            vec![
                (OpArg::Register, OpArg::Register),
                (OpArg::Register, OpArg::Immediate(Size::Byte))
            ]
        ),
    ]
    .into_iter()
    .collect();
}
