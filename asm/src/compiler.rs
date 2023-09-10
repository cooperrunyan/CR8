use std::collections::HashMap;

use crate::ast::{AddrByte, Ast, AstNode, Ident, Instruction, Label, Value};
use crate::expr;
use crate::op::Operation;

#[derive(Debug, Default)]
pub struct Compiler {
    pub bin: Vec<u8>,
    pub ast: Ast,
    pub labels: HashMap<String, usize>,
    pub last_label: String,
    pub pc: usize,
}

impl From<Ast> for Compiler {
    fn from(ast: Ast) -> Self {
        Self {
            ast,
            ..Default::default()
        }
    }
}

impl Compiler {
    pub fn strip_labels(&mut self) {
        for node in self.ast.tree.iter() {
            match node {
                AstNode::Label(Label::Label(ln)) => {
                    self.last_label = ln.to_string();
                    self.labels.insert(ln.to_owned(), self.pc);
                }
                AstNode::Label(Label::SubLabel(sub)) => {
                    self.labels
                        .insert(format!("{}{sub}", self.last_label), self.pc);
                }
                AstNode::Instruction(Instruction::Native(op, args)) => {
                    let size = match op.size(args) {
                        Ok(sz) => sz,
                        Err(e) => panic!("Argument error for {op:#?}: {e:#?}. At {node:#?}"),
                    };
                    self.pc += size as usize;
                }
                oth => panic!("Unexpected {oth:#?}. At {node:#?}"),
            }
        }
        dbg!(&self.labels);
    }

    pub fn resolve_static(&self, name: &str) -> Result<i128, ()> {
        let Some(stat) = self.ast.statics.get(name) else {
            return Err(());
        };
        return Ok(stat.to_owned() as i128);
    }

    pub fn resolve_ident(&self, ident: &Ident) -> Result<i128, ()> {
        match ident {
            Ident::Addr(a) => self.resolve_expr(&a).map_err(|_| ()),
            Ident::Static(s) => self.resolve_static(&s),
            Ident::PC => Ok(self.pc as i128),
            _ => Err(()),
        }
    }

    fn resolve_expr(&self, expr: &str) -> Result<i128, String> {
        expr::parse(expr, &self)
    }

    pub fn compile(&mut self) {
        use Operation::*;

        self.last_label = String::new();

        let mut tree = vec![];
        tree.append(&mut self.ast.tree);
        self.ast.tree = vec![];

        for node in tree {
            match node {
                AstNode::Label(Label::Label(ln)) => self.last_label = ln,
                AstNode::Instruction(Instruction::Native(op, args)) => {
                    let mut header = (op as u8) << 4;
                    let mut compiled_args: Vec<u8> = vec![];
                    let mut regn = 0;

                    for arg in args {
                        match arg {
                            Value::AddrByte(AddrByte::High(a)) => match self.resolve_expr(&a) {
                                Ok(a) => compiled_args.push((a >> 8) as u8),
                                Err(e) => panic!("Unknown address: {a:#?}. {e:#?}"),
                            },
                            Value::AddrByte(AddrByte::Low(a)) => match self.resolve_expr(&a) {
                                Ok(a) => compiled_args.push(a as u8),
                                Err(e) => panic!("Unknown address: {a:#?}. {e:#?}"),
                            },
                            Value::Register(r) => {
                                if regn > 0 {
                                    compiled_args.push(r as u8)
                                } else {
                                    header |= r as u8
                                }
                                regn += 1;
                            }
                            Value::Immediate(imm) => compiled_args.push(imm as u8),
                            Value::Ident(id) => match self.resolve_ident(&id) {
                                Ok(a) => {
                                    compiled_args.push(a as u8);
                                    if op == LW || op == SW {
                                        compiled_args.push((a >> 8) as u8);
                                    }
                                }
                                Err(()) => panic!("Unknown identifier: {id:#?}"),
                            },
                            Value::Expression(exp) => match self.resolve_expr(&exp) {
                                Err(e) => panic!("Failed to resolve: {exp:#?}. {e:#?}"),
                                Ok(v) => {
                                    compiled_args.push(v as u8);
                                    if op == LW || op == SW {
                                        compiled_args.push((v >> 8) as u8);
                                    }
                                }
                            },
                        }
                    }
                    match (op, regn, compiled_args.len()) {
                        (LW, 1, 0) => header |= 0b00001000,
                        (SW, 1, 2) => header |= 0b00001000,
                        (PUSH | JNZ, 0, 1) => header |= 0b00001000,
                        (MOV | OUT | IN | CMP | ADC | SBB | OR | NOR | AND, 1, 1) => {
                            header |= 0b00001000
                        }
                        _ => {}
                    };
                    self.bin.push(header);
                    self.bin.append(&mut compiled_args);
                }
                _ => {}
            }
        }
    }
}
