use anyhow::{anyhow, bail, Result};
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
use crate::op::{Operation, OperationArgAmt};

pub use config::*;

use self::lex::{ignore_whitespace, Item, LexableWith, Macro};

#[derive(Debug, Default)]
pub struct Compiler {
    bin: Vec<u8>,
    tree: Vec<Node>,
    preamble: Option<Vec<Node>>,
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
        let mut ctx = Self::default();

        ctx.push(
            Input::File("prelude".to_string()),
            Arc::new(env::current_dir().unwrap()),
        )
        .unwrap();

        ctx
    }

    pub fn compile(mut self) -> Result<Vec<u8>> {
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
                    if !ln.contains('.') {
                        self.last_label = ln.to_string();
                    }
                }
                Node::Instruction(inst) => {
                    let op = Operation::try_from(inst.id.as_str())
                        .map_err(|_| anyhow!("Invalid operation {:#?}", inst.id))?;
                    let mut header = (op as u8) << 2;
                    let arg_amt = OperationArgAmt::from_args(&inst.args)?;
                    header |= arg_amt as u8;

                    if arg_amt == OperationArgAmt::R0I1 && &inst.id == "jnz" {
                        self.bin.push(header);
                        continue;
                    }

                    let mut bin = match arg_amt {
                        OperationArgAmt::R1I0 => match &inst.args[..] {
                            &[Value::Register(r)] => vec![header, r as u8],
                            _ => bail!("Invalid args for {:#?}", inst.id),
                        },
                        OperationArgAmt::R2I0 => match &inst.args[..] {
                            &[Value::Register(r1), Value::Register(r2)] => {
                                vec![header, (r1 as u8) | ((r2 as u8) << 4)]
                            }
                            _ => bail!("Invalid args for {:#?}", inst.id),
                        },
                        OperationArgAmt::R0I1 => match inst.args.get(0) {
                            Some(Value::Immediate(imm8)) => {
                                vec![header, (*imm8 as u8)]
                            }
                            Some(Value::Expr(e)) => match op {
                                Operation::LW | Operation::SW => {
                                    let r = e.resolve(&self)?;
                                    vec![header, r as u8, (r >> 8) as u8]
                                }
                                _ => vec![header, e.resolve(&self)? as u8],
                            },
                            _ => bail!("Invalid args for {:#?}", inst.id),
                        },
                        OperationArgAmt::R1I1 => {
                            let mut reg = 0;
                            let mut trail = vec![];
                            for arg in inst.args {
                                match arg {
                                    Value::Register(r) => reg = r as u8,
                                    Value::Immediate(imm) => trail.push(imm as u8),
                                    Value::Expr(e) => match op {
                                        Operation::LW | Operation::SW => {
                                            let r = e.resolve(&self)?;
                                            trail.push(r as u8);
                                            trail.push((r >> 8) as u8);
                                        }
                                        _ => trail.push(e.resolve(&self)? as u8),
                                    },
                                    _ => {}
                                }
                            }
                            let mut b = vec![header, reg];
                            b.append(&mut trail);
                            b
                        }
                    };
                    self.bin.append(&mut bin);
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
            let (item, b) = Item::lex_with(buf, path.clone()).map_err(|e| {
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
    format!("{file_path}:{}:{}", lines + 1, col + 1)
}
