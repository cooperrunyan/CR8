use anyhow::{bail, Result};

use super::lex::{expect_complete, Lexable, Pragma};
use super::Input;

use indexmap::IndexMap;

use crate::{compiler::micro::control::RawControlSignal, op::Operation};

mod control;
mod lex;

#[derive(Debug, PartialEq, Eq)]
pub struct Micro(IndexMap<Operation, MicroInstruction>);

#[derive(Debug, PartialEq, Eq, Default)]
pub struct MicroInstruction {
    imm: Option<Vec<Vec<MicroSignal>>>,
    reg: Option<Vec<Vec<MicroSignal>>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeIdentifier {
    Immediate,
    Register,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MicroSignal {
    AddressBusWrite(AddressBusWriter),
    DataBusWrite(DataBusWriter),
    DataBusRead(DataBusReader),
    Alu(AluSignal),
    StackPointer(StackPointerSignal),
    ProgramCounter(ProgramCounterSignal),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddressBusWriter {
    ProgramCounter,
    StackPointer,
    XY,
    LhsRhs,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataBusReader {
    Flags,
    Io,
    Memory,
    MemoryBank,
    Lhs,
    Rhs,
    Device,
    /// Selects a reader based on the register ID that is in lhs
    Sel,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataBusWriter {
    /// Selects a writer based on the register ID that is in rhs
    Sel,
    Device,
    K,
    AluFlags,
    Alu,
    Memory,
    Io,
    Rhs,
    Operation,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataBusWriteSignal {
    Device,
    K,
    AluFlags,
    Alu,
    Memory,
    Io,
    Rhs,
    Operation,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AluSignal {
    Add,
    Sub,
    And,
    Or,
    Nor,
    Cmp,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProgramCounterSignal {
    Increment,
    Jump,
    JumpNotZero,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StackPointerSignal {
    Increment,
    Decrement,
}

pub fn compile(input: Input) -> Result<Vec<u8>> {
    let (buf, _) = input.source(None, None)?;
    let buf = buf.unwrap_or_default();

    let (prag, buf) = Pragma::lex(&buf)?;

    if prag != Pragma::Micro {
        bail!("Expected #![micro] at the beginning of a microcode file");
    }

    let (micro, buf) = Micro::lex(buf)?;

    expect_complete(buf)?;

    let binary = micro
        .0
        .into_iter()
        .flat_map(|(operation, variants)| {
            let header = (operation as u8) << 4;
            let r = variants.reg.map(|reg| (header, reg));
            let i = variants.imm.map(|imm| (header | 0b1000, imm));
            [r, i]
        })
        .flatten()
        .map(|(header, lines)| {
            let last = lines.len() - 1;
            lines
                .into_iter()
                .enumerate()
                .map(|(i, line)| {
                    control::ControlSignal::try_from(&line)
                        .map(|sig| {
                            let mut bits = RawControlSignal::from(sig);
                            if i == last {
                                bits.0[2] |= 1; // Set the CC flag on the last line of the instruction
                            }
                            bits
                        })
                        .map_err(|e| {
                            let op = Operation::try_from(header >> 4).unwrap();
                            let imm = header & 0b1000 == 0b1000;
                            let variant = if imm { "imm" } else { "reg" };
                            e.context(format!(
                                "Operation \"{}\"\nVariant \"{}\" \nSignal {}",
                                op.to_string(),
                                variant,
                                i
                            ))
                        })
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|lines| (header, lines))
        })
        .collect::<Result<Vec<_>, _>>()?;

    for i in 0..7 {
        for (header, chunk) in binary.iter() {
            let header = header >> 3;
            let msg = match chunk.get(i) {
                None => "".to_string(),
                Some(sig) => format!("{i:03b}{header:05b} | {}", sig),
            }
            .split("")
            .collect::<Vec<_>>()
            .join(" ");

            println!("{}", msg);
        }
    }

    Ok(vec![])
}

#[cfg(test)]
#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    use crate::compiler::lex::{Lexable, Pragma};
    let buf = include_str!("../../builtin/core/micro.asm");
    let (prag, buf) = Pragma::lex(buf)?;
    assert!(prag == Pragma::Micro);
    let (micro, _) = Micro::lex(buf)?;

    dbg!(micro);
    Ok(())
}
