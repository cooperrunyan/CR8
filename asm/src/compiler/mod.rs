use anyhow::{bail, Result};
use indexmap::IndexMap;
use path_clean::clean;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;

mod config;
mod debug;
pub mod lex;
mod resolver;

use crate::compiler::lex::{Node, Value};
use crate::op::Operation;

pub use config::*;

use self::lex::{ignore_whitespace, Item, Lexable, Macro};

#[derive(Debug)]
pub struct Compiler {
    bin: Vec<u8>,
    tree: Vec<Node>,
    preamble: Option<Vec<Node>>,
    markers: IndexMap<usize, String>,
    labels: IndexMap<String, usize>,
    last_label: String,
    pc: usize,
    files: Vec<Arc<PathBuf>>,
    macros: IndexMap<String, Macro>,
    statics: IndexMap<String, usize>,
    ram_locations: IndexMap<String, usize>,
    ram_length: usize,
    ram_origin: usize,
}

impl Compiler {
    pub fn new() -> Self {
        let mut ctx = Self {
            bin: vec![],
            tree: vec![],
            markers: IndexMap::new(),
            labels: IndexMap::new(),
            last_label: String::new(),
            pc: 0,
            preamble: None,
            files: vec![],
            macros: IndexMap::new(),
            statics: IndexMap::new(),
            ram_locations: IndexMap::new(),
            ram_length: 0,
            ram_origin: 0,
        };

        ctx.push(
            Input::File("core".to_string()),
            Arc::new(env::current_dir().unwrap()),
        )
        .unwrap();

        ctx
    }

    pub fn compile(mut self) -> Result<Vec<u8>> {
        use Operation::*;

        if let Some(mut preamble) = self.preamble {
            let mut old = self.tree;
            preamble.append(&mut old);
            self.tree = preamble;
            self.preamble = None;
        }

        self.resolve_macros()?;
        self.resolve_labels()?;

        self.last_label = String::new();

        let mut tree = vec![];
        tree.append(&mut self.tree);
        self.tree = vec![];

        for node in tree {
            match node {
                Node::Explicit(_, mut val) => self.bin.append(&mut val.0),
                Node::Label(ln) => {
                    if !ln.contains(".") {
                        self.last_label = ln.to_string();
                    }
                }
                Node::Instruction(inst) => {
                    let op = Operation::try_from(inst.id.as_str()).unwrap(); // FIXME
                    let mut header = (op as u8) << 4;
                    let mut compiled_args: Vec<u8> = vec![];
                    let mut regn = 0;

                    for arg in inst.args {
                        match arg {
                            Value::Register(r) => {
                                if regn > 0 {
                                    compiled_args.push(r as u8)
                                } else {
                                    header |= r as u8
                                }
                                regn += 1;
                            }
                            Value::Immediate(imm) => compiled_args.push(imm as u8),
                            Value::Expr(exp) => match exp.resolve(&self) {
                                Err(e) => bail!("Failed to resolve: {e:#?}"),
                                Ok(v) => {
                                    compiled_args.push(v as u8);
                                    if op == LW || op == SW {
                                        compiled_args.push((v >> 8) as u8);
                                    }
                                }
                            },
                            _ => {}
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
            let (content, path) = input.source(Some(&from), Some(&self.files))?;

            self.files.push(Arc::new(path));
            match content {
                Some(c) => c,
                None => return Ok(()),
            }
        };

        let mut buf = content.as_str();
        let mut nodes = vec![];
        let path = self.files.last().unwrap();

        loop {
            buf = ignore_whitespace(buf);
            if buf.is_empty() {
                break;
            }
            let (item, b) = Item::lex(buf).map_err(|e| {
                e.context(find_err_location(
                    buf,
                    &content,
                    clean(path.as_path()).display().to_string().as_str(),
                ))
            })?;
            nodes.push(item);
            buf = b;
        }

        self.resolve_directives(nodes)?;

        Ok(())
    }
}

pub fn find_err_location(at: &str, file_content: &str, file_path: &str) -> String {
    let mut lines = 0;
    let mut col = 0;
    let Some(idx) = file_content.find(at) else {
        return file_path.to_string();
    };

    for (i, ch) in file_content.chars().take(idx).enumerate() {
        if i == idx {
            break;
        }

        if ch == '\n' {
            lines += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    return format!("{file_path}:{}:{}", lines + 1, col + 1);
}
