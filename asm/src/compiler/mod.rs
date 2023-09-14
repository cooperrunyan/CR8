use std::collections::HashMap;
use std::fs;

mod ast;
mod config;
mod debug;
mod lexer;
mod resolver;
mod tokenizer;

use ast::{AstNode, Macro};
use lexer::Lexer;

use crate::compiler::ast::{AddrByte, Instruction, Label, Value};
use crate::op::Operation;

pub use config::Config;

use self::config::Input;

use super::std::STD;

#[derive(Debug)]
pub struct Compiler {
    bin: Vec<u8>,
    tree: Vec<AstNode>,
    labels: HashMap<String, usize>,
    last_label: String,
    pc: usize,
    files: Vec<String>,
    macros: HashMap<String, Macro>,
    statics: HashMap<String, u128>,
    ram_locations: HashMap<String, u128>,
    ram_length: u128,
    ram_origin: u16,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            bin: vec![],
            tree: vec![],
            labels: HashMap::new(),
            last_label: String::new(),
            pc: 0,
            files: vec![],
            macros: HashMap::new(),
            statics: HashMap::new(),
            ram_locations: HashMap::new(),
            ram_length: 0,
            ram_origin: 0,
        }
    }

    pub fn compile(mut self) -> Vec<u8> {
        use Operation::*;

        self.debug();

        self.resolve_macros();
        self.resolve_labels();

        self.last_label = String::new();

        let mut tree = vec![];
        tree.append(&mut self.tree);
        self.tree = vec![];

        for node in tree {
            match node {
                AstNode::Directive(ast::Directive::Rom(_, mut val)) => self.bin.append(&mut val),
                AstNode::Label(Label::Label(ln)) => self.last_label = ln,
                AstNode::Instruction(Instruction::Native(op, args)) => {
                    let mut header = (op as u8) << 4;
                    let mut compiled_args: Vec<u8> = vec![];
                    let mut regn = 0;

                    for arg in args {
                        match arg {
                            Value::AddrByte(AddrByte::High(a)) => match self.resolve_expr(&a) {
                                Ok(a) => compiled_args.push((a >> 8) as u8),
                                Err(e) => err!("Unknown address: {a:#?}. {e:#?}"),
                            },
                            Value::AddrByte(AddrByte::Low(a)) => match self.resolve_expr(&a) {
                                Ok(a) => compiled_args.push(a as u8),
                                Err(e) => err!("Unknown address: {a:#?}. {e:#?}"),
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
                                Err(()) => err!("Unknown identifier: {id:#?}"),
                            },
                            Value::Expression(exp) => match self.resolve_expr(&exp) {
                                Err(e) => err!("Failed to resolve: {exp:#?}. {e:#?}"),
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
                        (LW, 1, 2) => header |= 0b00001000,
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

        self.debug();

        self.bin
    }

    pub fn push(&mut self, input: Input) {
        let (source, file) = match input {
            Input::File(f) => {
                let f = if f.ends_with(".asm") {
                    f
                } else {
                    format!("{f}.asm")
                };
                let Ok(source) = fs::read_to_string(&f) else {
                    err!("Failed to read {f}");
                };
                (source, f)
            }
            Input::Raw(s) => (s, "raw".to_string()),
        };

        self.files.push(file.clone());

        let tokens = Compiler::tokenize(&source, &file);
        let nodes = match Lexer::new(tokens, &file).lex() {
            Ok(n) => n.nodes,
            Err(e) => err!("Error at file: {:?}\n{}", &file, e),
        };

        self.resolve_directives(nodes);
    }
}
