use anyhow::bail;
use indexmap::IndexMap;

use crate::compiler::lex::{
    collect_while, expect, ignore_comment, ignore_whitespace, ignore_whitespace_noline, LexResult,
    Lexable,
};
use crate::lex_enum;
use crate::op::Operation;

use super::{
    AddressBusWriter, AluSignal, DataBusReader, DataBusWriter, Micro, MicroInstruction,
    MicroSignal, ProgramCounterSignal, StackPointerSignal, TypeIdentifier,
};

const ADDRESS_BUS_WRITE: &str = "aw";
const DATA_BUS_WRITE: &str = "dw";
const DATA_BUS_READ: &str = "dr";
const PROGRAM_COUNTER: &str = "pc";
const STACK_POINTER: &str = "sp";
const ALU: &str = "alu";
const RESET: &str = "rst";

const SEL: &str = "sel";
const A: &str = "a";
const B: &str = "b";
const C: &str = "c";
const D: &str = "d";
const X: &str = "x";
const Y: &str = "y";
const Z: &str = "z";
const LHS: &str = "lhs";
const RHS: &str = "rhs";
const LR: &str = "lr";
const XY: &str = "xy";
const ALU_FLAGS: &str = "alflg";
const FLAGS: &str = "f";
const K: &str = "k";
const IO: &str = "io";
const MEMORY: &str = "mem";

const INC: &str = "++";
const DEC: &str = "--";

const PC_JUMP: &str = "j";
const PC_JNZ: &str = "jnz";

const ALU_ADD: &str = "add";
const ALU_SUB: &str = "sub";
const ALU_AND: &str = "and";
const ALU_OR: &str = "or";
const ALU_NOR: &str = "nor";
const ALU_CMP: &str = "cmp";

impl<'b> Lexable<'b> for Micro {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let mut micro = IndexMap::new();
        let mut buf = buf;
        loop {
            buf = ignore_whitespace(buf);
            if buf.is_empty() {
                return Ok((Self(micro), buf));
            }
            let (op, b) = Operation::lex(buf)?;
            buf = b;
            buf = ignore_whitespace(buf);
            buf = expect(buf, ":")?;
            let (instruction, b) = MicroInstruction::lex(buf)?;
            buf = b;
            if micro.insert(op, instruction).is_some() {
                bail!("Attempted to redefine microcode for {op:#?}");
            }
        }
    }
}

impl<'b> Lexable<'b> for MicroInstruction {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace(buf);
        let mut buf = expect(buf, "{")?;
        let mut inst = MicroInstruction::default();
        loop {
            buf = {
                let buf = buf;
                let buf = buf.trim_start_matches(char::is_whitespace);
                let buf = ignore_comment(buf);
                let buf = buf.trim_start_matches(char::is_whitespace);
                buf
            };
            if let Ok(buf) = expect(buf, "}") {
                return Ok((inst, buf));
            }
            buf = expect(buf, "(")?;
            let (id, b) = TypeIdentifier::lex(buf)?;
            buf = b;
            buf = ignore_whitespace(buf);
            buf = expect(buf, ")")?;
            buf = ignore_whitespace(buf);
            buf = expect(buf, "=>")?;
            buf = ignore_whitespace(buf);
            buf = expect(buf, "{")?;
            let mut lines = vec![];
            loop {
                buf = ignore_whitespace(buf);
                if let Ok(b) = expect(buf, "}") {
                    buf = b;
                    break;
                }
                let (line, b) = Vec::<MicroSignal>::lex(buf)?;
                buf = b;
                lines.push(line);
            }
            match id {
                TypeIdentifier::Immediate => {
                    if inst.imm.is_some() {
                        bail!("Attempted to set \"imm\" twice");
                    } else {
                        inst.imm = Some(lines);
                    }
                }
                TypeIdentifier::Register => {
                    if inst.reg.is_some() {
                        bail!("Attempted to set \"reg\" twice");
                    } else {
                        inst.reg = Some(lines);
                    }
                }
            }
        }
    }
}

impl<'b> Lexable<'b> for MicroSignal {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace_noline(buf);
        let (x, buf) = collect_while(buf, char::is_alphanumeric)?;

        match x {
            ADDRESS_BUS_WRITE => {
                let (writer, buf) = AddressBusWriter::lex(buf)?;
                Ok((Self::AddressBusWrite(writer), buf))
            }
            DATA_BUS_WRITE => {
                let (writer, buf) = DataBusWriter::lex(buf)?;
                Ok((Self::DataBusWrite(writer), buf))
            }
            DATA_BUS_READ => {
                let (reader, buf) = DataBusReader::lex(buf)?;
                Ok((Self::DataBusRead(reader), buf))
            }
            ALU => {
                let (alu_sig, buf) = AluSignal::lex(buf)?;
                Ok((Self::Alu(alu_sig), buf))
            }
            STACK_POINTER => {
                let (sp, buf) = StackPointerSignal::lex(buf)?;
                Ok((Self::StackPointer(sp), buf))
            }
            PROGRAM_COUNTER => {
                let (pc, buf) = ProgramCounterSignal::lex(buf)?;
                Ok((Self::ProgramCounter(pc), buf))
            }
            RESET => Ok((Self::Reset, buf)),
            o => bail!("Invalid signal {o:#?}"),
        }
    }
}

impl<'b> Lexable<'b> for AddressBusWriter {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace_noline(buf);
        let (x, buf) = collect_while(buf, char::is_alphanumeric)?;
        Ok((
            match x {
                PROGRAM_COUNTER => Self::ProgramCounter,
                STACK_POINTER => Self::StackPointer,
                XY => Self::XY,
                LR => Self::LhsRhs,
                o => bail!("Invalid operation {o:#?} for {ADDRESS_BUS_WRITE:#?}"),
            },
            buf,
        ))
    }
}

impl<'b> Lexable<'b> for DataBusReader {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace_noline(buf);
        let (x, buf) = collect_while(buf, char::is_alphanumeric)?;
        Ok((
            match x {
                SEL => Self::Sel,
                FLAGS => Self::Flags,
                MEMORY => Self::Memory,
                IO => Self::Io,
                RHS => Self::Rhs,
                LHS => Self::Lhs,
                LR => Self::LhsRhs,
                o => bail!("Invalid operation {o:#?} for {DATA_BUS_READ:#?}"),
            },
            buf,
        ))
    }
}

impl<'b> Lexable<'b> for DataBusWriter {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace_noline(buf);
        let (x, buf) = collect_while(buf, char::is_alphanumeric)?;
        Ok((
            match x {
                SEL => Self::Sel,
                A => Self::A,
                B => Self::B,
                C => Self::C,
                D => Self::D,
                X => Self::X,
                Y => Self::Y,
                Z => Self::Z,
                FLAGS => Self::Flags,
                K => Self::K,
                ALU_FLAGS => Self::AluFlags,
                ALU => Self::Alu,
                MEMORY => Self::Memory,
                IO => Self::Io,
                RHS => Self::Rhs,
                o => bail!("Invalid operation {o:#?} for {DATA_BUS_WRITE:#?}"),
            },
            buf,
        ))
    }
}

impl<'b> Lexable<'b> for AluSignal {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace_noline(buf);
        let (x, buf) = collect_while(buf, char::is_alphanumeric)?;
        Ok((
            match x {
                ALU_ADD => Self::Add,
                ALU_SUB => Self::Sub,
                ALU_AND => Self::And,
                ALU_OR => Self::Or,
                ALU_NOR => Self::Nor,
                ALU_CMP => Self::Cmp,
                o => bail!("Invalid operation {o:#?} for {ALU:#?}"),
            },
            buf,
        ))
    }
}

impl<'b> Lexable<'b> for ProgramCounterSignal {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace_noline(buf);
        let (x, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '+')?;
        Ok((
            match x {
                INC => Self::Increment,
                PC_JUMP => Self::Jump,
                PC_JNZ => Self::JumpNotZero,
                o => bail!("Invalid operation {o:#?} for {PROGRAM_COUNTER:#?}"),
            },
            buf,
        ))
    }
}

impl<'b> Lexable<'b> for StackPointerSignal {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace_noline(buf);
        let (x, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '+' || c == '-')?;
        Ok((
            match x {
                INC => Self::Increment,
                DEC => Self::Decrement,
                o => bail!("Invalid operation {o:#?} for {STACK_POINTER:#?}"),
            },
            buf,
        ))
    }
}

impl<'b> Lexable<'b> for TypeIdentifier {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace_noline(buf);
        lex_enum! { buf;
            "reg" => Self::Register,
            "imm" => Self::Immediate,
        }
    }
}
