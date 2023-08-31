use std::collections::HashMap;
use std::path::PathBuf;

use cfg::{op::Operation, reg::Register};

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

macro_rules! operator {
    ($n:ident, $t:expr) => {
        pub fn $n(self, rhs: Self) -> Expression {
            $t(vec![self, rhs])
        }
    };
}

impl_conv! {to_node, ToNode, AstNode}
impl_conv! {to_value, ToValue, Value}
impl_conv! {to_exp_item, ToExpressionItem, ExpressionItem}

#[derive(Debug, Default)]
pub struct Ast {
    pub tree: Vec<AstNode>,
    pub files: Vec<String>,
    pub macros: HashMap<String, Macro>,
    pub statics: HashMap<String, u128>,
    pub ram_locations: HashMap<String, u128>,
    pub ram_length: u128,
    pub ram_origin: u16,
}

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

#[derive(Debug)]
pub enum Value {
    Expression(Expression),
    Immediate(i128),
    Register(Register),
    Addr(Addr),
    Ident(Ident),
}

to_value! {Expression, Value::Expression}
to_value! {Register, Value::Register}
to_value! {Addr, Value::Addr}
to_value! {Ident, Value::Ident}
to_value! {i128, Value::Immediate}

#[derive(Debug)]
pub enum Addr {
    LowByte(String),
    HighByte(String),
}

#[derive(Debug)]
pub enum Ident {
    Static(String),
    Addr,
    MacroArg(String),
    PC,
}

#[derive(Debug)]
pub enum Expression {
    Add(Vec<ExpressionItem>),
    Mul(Vec<ExpressionItem>),
    Sub(Vec<ExpressionItem>),
    Div(Vec<ExpressionItem>),
    LeftShift(Vec<ExpressionItem>),
    RightShift(Vec<ExpressionItem>),
    And(Vec<ExpressionItem>),
    Or(Vec<ExpressionItem>),
}

#[derive(Debug)]
pub enum ExpressionItem {
    Immediate(i128),
    Ident(Ident),
    Group(Expression),
}

to_exp_item! {Ident, ExpressionItem::Ident}
to_exp_item! {Expression, ExpressionItem::Group}
to_exp_item! {i128, ExpressionItem::Immediate}

impl ExpressionItem {
    operator! {sub, Expression::Sub}
    operator! {add, Expression::Add}
    operator! {mul, Expression::Mul}
    operator! {div, Expression::Div}
    operator! {left_shift, Expression::LeftShift}
    operator! {right_shift, Expression::RightShift}
    operator! {and, Expression::And}
    operator! {or, Expression::Or}
}

impl From<Vec<AstNode>> for Ast {
    fn from(value: Vec<AstNode>) -> Self {
        Self {
            tree: value,
            ..Default::default()
        }
    }
}
