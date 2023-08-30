use std::collections::HashMap;
use std::path::PathBuf;

use cfg::{op::Operation, reg::Register};

#[derive(Debug, Default)]
pub struct Ast<'c> {
    tree: Vec<AstNode<'c>>,
    files: Vec<PathBuf>,
    macros: HashMap<String, Macro<'c>>,
    statics: HashMap<String, u128>,
    ram_origin: u16,
}

#[derive(Debug)]
pub enum AstNode<'n> {
    Macro(Macro<'n>),
    Label(&'n str),
    SubLabel(&'n str),
    RamOriginDef(u128),
    RamAlloc(String, u128),
    Rom(String, Vec<u8>),
    StaticDef(String, u128),
    Import(String),
    Instruction(Instruction<'n>),
}

#[derive(Debug)]
pub struct Macro<'m> {
    name: &'m str,
    args: Vec<MacroArg<'m>>,
    body: Vec<Instruction<'m>>,
}

#[derive(Debug)]
pub enum MacroArg<'m> {
    Immediate(&'m str),
    Register(&'m str),
    ImmReg(&'m str),
    Addr(&'m str),
}

#[derive(Debug)]
pub enum Instruction<'i> {
    Native(Operation, Vec<Value<'i>>),
    Macro(&'i str, Vec<Value<'i>>),
}

#[derive(Debug)]
pub enum Value<'a> {
    Expression(ExpOperator<'a>),
    Immediate(i128),
    Register(Register),
    Addr(Addr<'a>),
    Ident(Ident<'a>),
}

#[derive(Debug)]
pub enum Addr<'a> {
    LowByte(&'a str),
    HighByte(&'a str),
}

#[derive(Debug)]
pub enum Ident<'a> {
    Static(&'a str),
    Addr(&'a str),
    MacroArg(&'a str),
    PC,
}

#[derive(Debug)]
pub enum ExpOperator<'e> {
    Add(Vec<ExpItem<'e>>),
    Mul(Vec<ExpItem<'e>>),
    Sub(Vec<ExpItem<'e>>),
    Div(Vec<ExpItem<'e>>),
    LeftShift(Vec<ExpItem<'e>>),
    RightShift(Vec<ExpItem<'e>>),
    And(Vec<ExpItem<'e>>),
    Or(Vec<ExpItem<'e>>),
}

#[derive(Debug)]
pub enum ExpItem<'a> {
    Immediate(i128),
    Ident(Ident<'a>),
    Group(ExpOperator<'a>),
}

impl<'c> From<Vec<AstNode<'c>>> for Ast<'c> {
    fn from(value: Vec<AstNode<'c>>) -> Self {
        Self {
            tree: value,
            ..Default::default()
        }
    }
}

impl<'a> Ast<'a> {
    pub fn insert(&mut self, mut nodes: Vec<AstNode<'a>>) {
        nodes.append(&mut self.tree);
        self.tree = nodes;
    }
}
