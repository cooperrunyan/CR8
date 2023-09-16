use crate::{op::Operation, reg::Register};

macro_rules! impl_conv {
    ($nm:ident, $trait:ident, $to:ident) => {
        pub trait $trait {
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
pub enum AstNode {
    Directive(Directive),
    Label(Label),
    Instruction(Instruction),
}

to_node! {Directive, AstNode::Directive}
to_node! {Instruction, AstNode::Instruction}
to_node! {Label, AstNode::Label}

#[derive(Debug)]
pub enum Label {
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
pub enum Directive {
    Macro(Macro),
    Origin(u128),
    Dynamic(String, u128),
    Rom(String, Vec<u8>),
    Define(String, u128),
    Import(String),
    Marker(String),
}

#[derive(Debug)]
pub struct Macro {
    pub name: String,
    pub args: Vec<MacroArg>,
    pub body: Vec<Instruction>,
}

impl ToNode for Macro {
    fn to_node(self) -> AstNode {
        self.to_directive().to_node()
    }
}

impl Macro {
    pub fn new(name: String, args: Vec<MacroArg>, body: Vec<Instruction>) -> Self {
        Self { name, args, body }
    }

    fn to_directive(self) -> Directive {
        Directive::Macro(self)
    }
}

#[derive(Debug)]
pub enum MacroArg {
    Immediate(String),
    Register(String),
    ImmReg(String),
    Addr(String),
}

#[derive(Debug)]
pub enum Instruction {
    Native(Operation, Vec<Value>),
    Macro(String, Vec<Value>),
}

#[derive(Debug, Clone)]
pub enum Value {
    Expression(String),
    Immediate(i128),
    Register(Register),
    AddrByte(AddrByte),
    Ident(Ident),
}

to_value! {AddrByte, Value::AddrByte}
to_value! {Ident, Value::Ident}
to_value! {Register, Value::Register}
to_value! {i128, Value::Immediate}

#[derive(Debug, Clone)]
pub enum AddrByte {
    Low(String),
    High(String),
}

#[derive(Debug, Clone)]
pub enum Ident {
    Static(String),
    Addr(String),
    MacroArg(String),
}
