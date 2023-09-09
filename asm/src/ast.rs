use std::collections::HashMap;
use std::fs;

use crate::lex::lex;
use crate::token::tokenize;
use crate::STD;

use super::{op::Operation, reg::Register};

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

#[derive(Debug, Clone)]
pub enum Value {
    Expression(Expression),
    Immediate(i128),
    Register(Register),
    AddrByte(AddrByte),
    Ident(Ident),
}

to_value! {Expression, Value::Expression}
to_value! {Register, Value::Register}
to_value! {AddrByte, Value::AddrByte}
to_value! {Ident, Value::Ident}
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
    PC,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

impl Ast {
    pub fn start(source: String, file: String) -> Self {
        let tokens = tokenize(&source, &file);
        let nodes = match lex(tokens, &file) {
            Ok(n) => n,
            Err(e) => panic!("Error at file: {:?}:{}:\n\n{}", e.file, e.line, e.msg),
        };

        let mut ctx = Ast::default();

        ctx.strip(nodes);
        ctx = ctx.fill_macros();
        ctx
    }

    fn strip(&mut self, nodes: Vec<AstNode>) {
        for node in nodes {
            match node {
                AstNode::Directive(Directive::Import(f)) => {
                    if self.files.contains(&f) {
                        continue;
                    }

                    let file = if f.starts_with("<std>") {
                        if let Some(file) = STD.get(&f) {
                            file.to_string()
                        } else {
                            panic!("Attempted to import non-existent <std> file: {f:#?}")
                        }
                    } else {
                        if let Ok(file) = fs::read_to_string(&f) {
                            file
                        } else {
                            panic!("Unresolved import: {f:#?}")
                        }
                    };

                    let tokens = tokenize(&file, &f);
                    let nodes = match lex(tokens, &f) {
                        Ok(n) => n,
                        Err(e) => panic!("Error at file: {}:{}\n\n{}", e.file, e.line, e.msg),
                    };

                    self.files.push(f);

                    self.strip(nodes);
                }
                AstNode::Directive(Directive::Define(k, v)) => {
                    if self.statics.contains_key(&k) {
                        panic!("Error: attempted to define {k} twice");
                    }
                    self.statics.insert(k, v);
                }
                AstNode::Directive(Directive::Dynamic(k, v)) => {
                    if self.ram_locations.contains_key(&k) {
                        panic!("Error: attempted to set #dyn {k:#?} twice");
                    }
                    self.ram_locations.insert(k, self.ram_length);
                    self.ram_length += v;
                }
                AstNode::Directive(Directive::Origin(v)) => {
                    self.ram_origin = v as u16;
                }
                AstNode::Directive(Directive::Macro(m)) => {
                    if self.macros.contains_key(&m.name) {
                        panic!("Error: attempted to set macro {:#?} twice", m.name);
                    }

                    self.macros.insert(m.name.clone(), m);
                }
                oth => self.tree.push(oth),
            }
        }
    }

    fn fill_macros(mut self) -> Ast {
        let mut new_tree = vec![];

        let tree = self.tree;

        for node in tree {
            let mut stripped = swap_macro(&self.macros, node);
            new_tree.append(&mut stripped);
        }

        self.tree = new_tree;

        self
    }
}

fn swap_macro(macros: &HashMap<String, Macro>, node: AstNode) -> Vec<AstNode> {
    let mut tree = vec![];

    match node {
        AstNode::Instruction(Instruction::Macro(mac_name, mut args)) => {
            args.reverse();
            let mac_content = match macros.get(&mac_name) {
                Some(m) => m,
                None => panic!("Macro '{mac_name}' not defined"),
            };

            let mut parsed_args: HashMap<String, Value> = HashMap::new();

            for (i, mac_arg) in mac_content.args.iter().enumerate() {
                match mac_arg {
                    MacroArg::Immediate(name) => {
                        let Some(next) = args.pop() else {
                            panic!("Bad amount of arguments sent to macro: '{mac_name:#?}'. Expected immediate, found none.");
                        };
                        match next {
                            Value::Immediate(v) => parsed_args.insert(name.to_string(),  Value::Immediate(v)),
                            Value::Ident(id) => parsed_args.insert(name.to_string(), Value::Ident(id)),
                            _ => panic!("Expected an immediate value at {mac_name:#?} argument {i}. Received: {next:#?}")
                        };
                    }
                    MacroArg::Register(name) => {
                        let Some(next) = args.pop() else {
                            panic!("Bad amount of arguments sent to macro: '{mac_name:#?}'. Expected register, found none.");
                        };
                        match next {
                            Value::Register(r) => parsed_args.insert(name.to_string(),  Value::Register(r)),
                            _ => panic!("Expected a register at {mac_name:#?} argument {i}. Received: {next:#?}")
                        };
                    }
                    MacroArg::ImmReg(name) => {
                        let Some(next) = args.pop() else {
                            panic!("Bad amount of arguments sent to macro: '{mac_name:#?}'. Expected Immediate or Register, found none.");
                        };
                        match next {
                            Value::Immediate(v) => parsed_args.insert(name.to_string(),  Value::Immediate(v)),
                            Value::Register(r) => parsed_args.insert(name.to_string(),  Value::Register(r)),
                            Value::Ident(id) => parsed_args.insert(name.to_string(), Value::Ident(id)),
                            _ => panic!("Expected an immediate or register at {mac_name:#?} argument {i}. Received: {next:#?}")
                        };
                    }
                    MacroArg::Addr(name) => {
                        let Some(next) = args.pop() else {
                            panic!("Bad amount of arguments sent to macro: '{mac_name:#?}'. Expected an address, found none.");
                        };
                        match next {
                            Value::Ident(Ident::Addr(a)) => {
                                parsed_args.insert(format!("{name}"), Value::Ident(Ident::Addr(a.clone())));
                                parsed_args.insert(format!("{name}l"), Value::AddrByte(AddrByte::Low(a.clone())));
                                parsed_args.insert(format!("{name}h"), Value::AddrByte(AddrByte::High(a)));
                            },
                            _ => panic!("Expected an address at {mac_name:#?} argument {i}. Received: {next:#?}")
                        };
                    }
                }
            }
            for instruction in mac_content.body.iter() {
                let (empty, args) = match instruction {
                    Instruction::Macro(m, args) => {
                        (Instruction::Macro(m.to_string(), vec![]), args)
                    }
                    Instruction::Native(n, args) => {
                        (Instruction::Native(n.to_owned(), vec![]), args)
                    }
                };
                let mut new_args: Vec<Value> = vec![];

                for arg in args {
                    match arg {
                        Value::Ident(Ident::MacroArg(ma)) => {
                            let Some(val) = parsed_args.get(ma) else {
                                panic!("Attempted to use undefined macro arg at {mac_name:#?} {empty:#?}");
                            };
                            new_args.push(val.clone().to_owned());
                        }
                        oth => new_args.push(oth.clone()),
                    }
                }

                match instruction {
                    Instruction::Macro(m, _) => {
                        let mut nodes = swap_macro(
                            macros,
                            Instruction::Macro(m.to_owned(), new_args).to_node(),
                        );
                        tree.append(&mut nodes);
                    }
                    Instruction::Native(n, _) => {
                        tree.push(Instruction::Native(n.to_owned(), new_args).to_node())
                    }
                };
            }
        }
        _ => tree.push(node),
    };

    tree
}
