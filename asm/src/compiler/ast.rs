use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Arc;

use crate::{op::Operation, reg::Register};

macro_rules! impl_conv {
    ($nm:ident, $trait:ident, $to:ident) => {
        pub(crate) trait $trait {
            fn $nm(self) -> $to;
        }

        macro_rules! $nm {
            ($from:ident,$wrap:expr) => {
                impl $trait for $from {
                    fn $nm(self) -> $to {
                        $wrap(self)
                    }
                }
            };
        }
    };
}

impl_conv! {to_node, ToNode, AstNode}
impl_conv! {to_value, ToValue, Value}

#[derive(Debug)]
pub(crate) enum AstNode {
    Directive(Directive),
    Label(Label),
    Instruction(Instruction),
}

to_node! {Directive, AstNode::Directive}
to_node! {Instruction, AstNode::Instruction}
to_node! {Label, AstNode::Label}

#[derive(Debug)]
pub(crate) enum Label {
    Label(String),
    SubLabel(String),
}

impl From<String> for Label {
    fn from(value: String) -> Self {
        if value.starts_with('.') {
            Self::SubLabel(value)
        } else {
            Self::Label(value)
        }
    }
}

#[derive(Debug)]
pub(crate) enum Directive {
    Macro(Macro),
    Preamble(Vec<AstNode>),
    DynamicOrigin(u128),
    Dynamic(String, u128),
    Rom(String, Vec<u8>),
    Define(String, u128),
    Import(String, Arc<PathBuf>),
}

#[derive(Debug)]
pub(crate) struct Macro {
    pub(crate) name: String,
    pub(crate) captures: Vec<Capture>,
}

#[derive(Debug)]
pub(crate) struct Capture {
    pub(crate) args: Vec<MacroArg>,
    pub(crate) body: Vec<Instruction>,
}

impl ToNode for Macro {
    fn to_node(self) -> AstNode {
        self.to_directive().to_node()
    }
}

impl Macro {
    pub(crate) fn new(name: String, captures: Vec<Capture>) -> Self {
        Self { name, captures }
    }

    fn to_directive(self) -> Directive {
        Directive::Macro(self)
    }
}

#[derive(Debug)]
pub(crate) enum MacroArg {
    Imm8(String),
    Register(String),
    ImmReg(String),
    Imm16(String),
}

impl Display for MacroArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Imm16(a) | Self::ImmReg(a) | Self::Imm8(a) | Self::Register(a) => {
                f.write_fmt(format_args!("{a}"))
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum Instruction {
    Native(Operation, Vec<Value>),
    Macro(String, Vec<Value>),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = match self {
            Self::Native(i, a) => {
                f.write_fmt(format_args!("{i} "))?;
                a
            }
            Self::Macro(m, a) => {
                f.write_fmt(format_args!("{m} "))?;
                a
            }
        };

        for (i, arg) in args.iter().enumerate() {
            f.write_fmt(format_args!("{arg}"))?;

            if i != args.len() - 1 {
                f.write_str(", ")?
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Value {
    Expression(String),
    Immediate(i128),
    Register(Register),
    AddrByte(AddrByte),
    MacroArg(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddrByte(a) => a.fmt(f),
            Self::MacroArg(a) => a.fmt(f),
            Self::Register(r) => f.write_fmt(format_args!("%{r}")),
            Self::Immediate(i) => {
                if *i > 0xff {
                    f.write_fmt(format_args!("{i:#06x}"))
                } else {
                    f.write_fmt(format_args!("{i:#04x}"))
                }
            }
            Self::Expression(e) => f.write_fmt(format_args!("[{e}]")),
        }
    }
}

to_value! {AddrByte, Value::AddrByte}
to_value! {Register, Value::Register}
to_value! {i128, Value::Immediate}

#[derive(Debug, Clone)]
pub(crate) enum AddrByte {
    Low(String),
    High(String),
}

impl Display for AddrByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low(a) => f.write_fmt(format_args!("[{a} & 0x00FF]")),
            Self::High(a) => f.write_fmt(format_args!("[{a} >> 8]")),
        }
    }
}
