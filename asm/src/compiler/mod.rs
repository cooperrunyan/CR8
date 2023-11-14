#![doc(alias = "assembler")]

use anyhow::{anyhow, bail, Result};
use indexmap::IndexMap;
use path_clean::clean;
use std::env;
use std::sync::Arc;
use std::{io::Write, path::PathBuf};

mod config;
mod debug;
pub mod lex;
pub mod micro;
mod resolver;

use crate::compiler::lex::Node;
use crate::op::Operation;

pub use config::*;

use self::lex::{ignore_whitespace, Item, LexableWith, Macro};

#[derive(Debug, Default)]
pub struct Compiler {
    pub bin: Vec<u8>,
    tree: Vec<Node>,
    preamble: bool,
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
            Input::File("core".to_string()),
            Arc::new(env::current_dir().unwrap()),
        )
        .unwrap();

        ctx
    }

    pub fn compile(&mut self) -> Result<()> {
        self.resolve_macros()?;
        self.resolve_labels()?;

        self.last_label = String::new();

        let mut tree = vec![];
        tree.append(&mut self.tree);
        self.tree = vec![];

        for node in tree {
            match node {
                Node::Constant(_, mut val) => self.bin.append(&mut val.0),
                Node::Label(ln) => {
                    if !ln.contains('.') {
                        self.last_label = ln.to_string();
                    }
                }
                Node::Instruction(inst) => {
                    let op = Operation::try_from(inst.id.as_str())
                        .map_err(|_| anyhow!("Invalid operation {:#?}", inst.id))?;

                    self.bin.append(&mut op.compile(inst.args, self)?);
                }
                _ => {}
            }
        }

        Ok(())
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

        self.resolve_meta(nodes)?;

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

// v3.0 hex words addressed
// 00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
// 10: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
// ...
pub fn logisim_hex_file<W: Write + Sync>(
    bin: &[u8],
    addr_width: usize,
    file: &mut W,
) -> Result<()> {
    const ROW: usize = 16;

    let addr_fmt = match addr_width {
        4 => |byte: &usize| format!("{byte:01x}"),
        8 => |byte: &usize| format!("{byte:02x}"),
        16 => |byte: &usize| format!("{byte:04x}"),
        32 => |byte: &usize| format!("{byte:08x}"),
        w => bail!("Invalid address width: {w}"),
    };

    file.write_all("v3.0 hex words addressed".as_bytes())?;

    for (i, bytes) in bin.chunks(ROW).enumerate() {
        let addr = i * ROW;
        let row = bytes
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ");
        file.write_fmt(format_args!("\n{}: {}", addr_fmt(&addr), row))?;
    }
    Ok(())
}
