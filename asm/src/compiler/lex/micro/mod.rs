use indexmap::IndexMap;

use crate::op::Operation;

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

#[derive(Debug, PartialEq, Eq)]
pub enum MicroSignal {
    AddressBusWrite(AddressBusWriter),
    DataBusWrite(DataBusWriter),
    DataBusRead(DataBusReader),
    Alu(AluSignal),
    StackPointer(StackPointerSignal),
    ProgramCounter(ProgramCounterSignal),
    Reset,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddressBusWriter {
    ProgramCounter,
    StackPointer,
    XY,
    LhsRhs,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DataBusReader {
    Flags,
    Io,
    Memory,
    Lhs,
    Rhs,
    LhsRhs, // puts high nibble into rhs and low into lhs
    Sel,    // Selects a reader based on the register ID that is in lhs
}

#[derive(Debug, PartialEq, Eq)]
pub enum DataBusWriter {
    Sel, // Selects a reader based on the register ID that is in rhs
    A,
    B,
    C,
    D,
    X,
    Y,
    Z,
    Flags,
    K,
    AluFlags,
    Alu,
    Memory,
    Io,
    Rhs,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AluSignal {
    Add,
    Sub,
    And,
    Or,
    Nor,
    Cmp,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramCounterSignal {
    Increment,
    Jump,
    JumpNotZero,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StackPointerSignal {
    Increment,
    Decrement,
}

#[cfg(test)]
#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    use crate::compiler::lex::{Lexable, Pragma};
    let buf = include_str!("../../../builtin/core/micro.asm");
    let (prag, buf) = Pragma::lex(buf)?;
    assert!(prag == Pragma::Micro);
    let (micro, _) = Micro::lex(buf)?;

    dbg!(micro);
    Ok(())
}
