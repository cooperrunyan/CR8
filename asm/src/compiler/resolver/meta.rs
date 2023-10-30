use anyhow::{bail, Result};

use super::Compiler;
use crate::compiler::config::Input;
use crate::compiler::lex::{Expr, Instruction, Item, ItemInner, Meta, Node, Value};

impl Compiler {
    pub(crate) fn resolve_meta(&mut self, nodes: Vec<Item>) -> Result<()> {
        for node in nodes {
            match node.item {
                ItemInner::Meta(Meta::Use(f)) => {
                    self.push(Input::File(f.to_string()), node.file)?;
                }
                ItemInner::Meta(Meta::Main(to)) => {
                    if self.preamble {
                        bail!("Cannot set #[main] twice");
                    }

                    self.tree.insert(
                        0,
                        Node::Instruction(Instruction {
                            id: "jmp".to_string(),
                            args: vec![Value::Expr(Expr::Variable(to))],
                        }),
                    );
                }
                ItemInner::Meta(Meta::Static(k, v)) => {
                    if self.statics.contains_key(&k) {
                        bail!("Error: attempted to define {k} twice");
                    }
                    self.statics.insert(k, v);
                }
                ItemInner::Meta(Meta::Dyn(k, v)) => {
                    if self.ram_locations.contains_key(&k) {
                        bail!("Error: attempted to set #[dyn] {k:#?} twice");
                    }
                    self.ram_locations
                        .insert(k, self.ram_length + self.ram_origin);
                    self.ram_length += v;
                }
                ItemInner::Meta(Meta::DynOrigin(v)) => {
                    self.ram_origin = v;
                }
                ItemInner::Meta(Meta::Macro(m)) => {
                    if self.macros.contains_key(&m.id) {
                        bail!("Error: attempted to set macro {:#?} twice", m.id);
                    }

                    self.macros.insert(m.id.to_string(), m);
                }
                ItemInner::Meta(Meta::Constant(id, bytes)) => {
                    self.tree.push(Node::Constant(id, bytes));
                }
                ItemInner::Node(n) => self.tree.push(n),
            }
        }

        Ok(())
    }
}
