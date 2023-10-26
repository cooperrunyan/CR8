use indexmap::IndexMap;

use crate::op::{Operation, OperationArgAmt};

mod lex;

#[derive(Debug)]
pub struct Micro(IndexMap<Operation, MicroInstruction>);

#[derive(Debug)]
pub struct MicroInstruction(IndexMap<OperationArgAmt, Vec<Vec<MicroSignal>>>);

#[derive(Debug)]
pub enum MicroSignal {
    AddressBusWrite(AddressBusWriter),
    DataBusWrite(DataBusWriter),
    DataBusRead(DataBusReader),
    Alu(AluSignal),
    StackPointer(StackPointerSignal),
    ProgramCounter(ProgramCounterSignal),
    Reset,
}

#[derive(Debug)]
pub enum AddressBusWriter {
    ProgramCounter,
    StackPointer,
    XY,
    LhsRhs,
}

#[derive(Debug)]
pub enum DataBusReader {
    Flags,
    Io,
    Memory,
    Lhs,
    Rhs,
    LhsRhs, // puts high nibble into rhs and low into lhs
    Sel,    // Selects a reader based on the register ID that is in lhs
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum AluSignal {
    Add,
    Sub,
    And,
    Or,
    Nor,
    Cmp,
}

#[derive(Debug)]
pub enum ProgramCounterSignal {
    Increment,
    Jump,
    JumpNotZero,
}

#[derive(Debug)]
pub enum StackPointerSignal {
    Increment,
    Decrement,
}

#[cfg(test)]
#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    use crate::compiler::lex::Lexable;

    let (microcode, _) = Micro::lex(include_str!("../../../builtin/core/micro.asm"))?;

    dbg!(microcode);
    Ok(())
}
