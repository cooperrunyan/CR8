use anyhow::{bail, Result};

use super::Compiler;
use crate::compiler::config::Input;
use crate::compiler::lex::{Expr, ExprOperation, Instruction, Item, ItemInner, Meta, Node, Value};

impl Compiler {
    pub(crate) fn resolve_meta(&mut self, nodes: Vec<Item>) -> Result<()> {
        for node in nodes {
            match node.item {
                ItemInner::Meta(Meta::Use(f)) => {
                    self.push(Input::File(f.to_string()), node.file)?;
                }
                ItemInner::Meta(Meta::Main(to)) => {
                    if self.preamble.is_some() {
                        bail!("Cannot set #[main] twice");
                    }

                    self.preamble = Some(vec![
                        Node::Instruction(Instruction {
                            id: "mov".to_string(),
                            args: vec![
                                Value::Register(crate::reg::Register::X),
                                Value::Expr(Expr::Expr {
                                    lhs: Box::new(Expr::Variable(to.clone())),
                                    op: ExprOperation::And,
                                    rhs: Box::new(Expr::Literal(0xFF)),
                                }),
                            ],
                        }),
                        Node::Instruction(Instruction {
                            id: "mov".to_string(),
                            args: vec![
                                Value::Register(crate::reg::Register::Y),
                                Value::Expr(Expr::Expr {
                                    lhs: Box::new(Expr::Variable(to.clone())),
                                    op: ExprOperation::Rsh,
                                    rhs: Box::new(Expr::Literal(8)),
                                }),
                            ],
                        }),
                        Node::Instruction(Instruction {
                            id: "jnz".to_string(),
                            args: vec![Value::Literal(1)],
                        }),
                    ]);
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
