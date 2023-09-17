use anyhow::{bail, Result};
use path_absolutize::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

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

pub use config::*;

use super::std::STD;

#[derive(Debug)]
pub struct Compiler {
    bin: Vec<u8>,
    tree: Vec<AstNode>,
    preamble: Option<Vec<AstNode>>,
    markers: HashMap<usize, String>,
    labels: HashMap<String, usize>,
    last_label: String,
    pc: usize,
    files: Vec<Arc<PathBuf>>,
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
            markers: HashMap::new(),
            labels: HashMap::new(),
            last_label: String::new(),
            pc: 0,
            preamble: None,
            files: vec![],
            macros: HashMap::new(),
            statics: HashMap::new(),
            ram_locations: HashMap::new(),
            ram_length: 0,
            ram_origin: 0,
        }
    }

    pub fn compile(mut self) -> Result<Vec<u8>> {
        use Operation::*;

        if let Some(mut preamble) = self.preamble {
            let mut old = self.tree;
            preamble.append(&mut old);
            self.tree = preamble;
            self.preamble = None;
        }

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
                    let mut marker = format!("{op} ");
                    for (i, arg) in args.iter().enumerate() {
                        marker.push_str(&format!("{arg}"));
                        if i != args.len() - 1 {
                            marker.push_str(&format!(", "));
                        }
                    }
                    self.markers.insert(self.bin.len(), marker);

                    let mut header = (op as u8) << 4;
                    let mut compiled_args: Vec<u8> = vec![];
                    let mut regn = 0;

                    for arg in args {
                        match arg {
                            Value::AddrByte(AddrByte::High(a)) => match self.resolve_expr(&a) {
                                Ok(a) => compiled_args.push((a >> 8) as u8),
                                Err(e) => bail!("Unknown address: {a:#?}. {e:#?}"),
                            },
                            Value::AddrByte(AddrByte::Low(a)) => match self.resolve_expr(&a) {
                                Ok(a) => compiled_args.push(a as u8),
                                Err(e) => bail!("Unknown address: {a:#?}. {e:#?}"),
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
                                Err(()) => bail!("Unknown identifier: {id:#?}"),
                            },
                            Value::Expression(exp) => match self.resolve_expr(&exp) {
                                Err(e) => bail!("Failed to resolve: {exp:#?}. {e:#?}"),
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

        Ok(self.bin)
    }

    pub fn push(&mut self, input: Input, from: Arc<PathBuf>) -> Result<()> {
        let content = {
            let (content, path) = match input {
                Input::File(path) => {
                    if path.starts_with("<std>") {
                        for included in self.files.iter() {
                            if included.clone() == PathBuf::from(&path).into() {
                                return Ok(());
                            }
                        }
                        if let Some(content) = STD.get(&path) {
                            (content.to_string(), path.into())
                        } else {
                            bail!("Attempted to import non-existent std file: {path:#?}");
                        }
                    } else {
                        let pb = PathBuf::from(&path);
                        let real = if pb.exists() && pb.is_file() {
                            pb
                        } else {
                            let possibilities = vec![
                                from.parent().unwrap_or(&from).join(&pb),
                                from.parent()
                                    .unwrap_or(&from)
                                    .join(&pb)
                                    .with_extension("asm"),
                                from.parent().unwrap_or(&from).join(&pb).join("mod.asm"),
                                from.parent().unwrap_or(&from).join(&pb).join("main.asm"),
                                pb.with_extension("asm"),
                                pb.join("main.asm"),
                                pb.join("mod.asm"),
                                pb,
                            ];

                            let mut found = None;

                            for possible in possibilities.iter() {
                                if possible.exists() && possible.is_file() {
                                    found = Some(possible.to_owned());
                                    break;
                                }
                            }
                            match found {
                                Some(p) => p,
                                None => {
                                    let attempted = possibilities
                                        .into_iter()
                                        .map(|p| {
                                            p.absolutize()
                                                .map(|p| p.to_str().unwrap_or("").to_string())
                                                .unwrap_or(String::new())
                                        })
                                        .collect::<Vec<_>>();
                                    bail!("Could not locate {path} in any of: \n  {attempted:#?}");
                                }
                            }
                        };

                        if let Ok(file) = fs::read_to_string(&real) {
                            (file, real)
                        } else {
                            bail!("Failed to read {real:?}");
                        }
                    }
                }

                Input::Raw(s) => (s, "raw".into()),
            };

            self.files.push(Arc::new(path));
            content
        };
        let nodes = {
            let path = self.files.last().unwrap();

            let tokens = Compiler::tokenize(content, path.clone())?;
            Lexer::new(tokens, path.clone()).lex()?.nodes
        };

        self.resolve_directives(nodes)?;

        Ok(())
    }
}
