use anyhow::{bail, Result};

use super::Compiler;
use crate::compiler::config::Input;
use crate::compiler::lex::{Directive, Expr, ExprOperation, Instruction, Item, Node, Value};

impl Compiler {
    pub(crate) fn resolve_directives(&mut self, nodes: Vec<Item>) -> Result<()> {
        for node in nodes {
            match node {
                Item::Directive(Directive::Use(f)) => {
                    let from = self.files.last().unwrap().clone();
                    self.push(Input::File(f.to_string()), from)?;
                }
                Item::Directive(Directive::Boot(to)) => {
                    if self.preamble.is_some() {
                        bail!("Cannot set #[boot] twice");
                    }

                    self.preamble = Some(vec![
                        Node::Instruction(Instruction {
                            id: "mov".to_string(),
                            args: vec![
                                Value::Register(crate::reg::Register::L),
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
                                Value::Register(crate::reg::Register::H),
                                Value::Expr(Expr::Expr {
                                    lhs: Box::new(Expr::Variable(to.clone())),
                                    op: ExprOperation::Rsh,
                                    rhs: Box::new(Expr::Literal(8)),
                                }),
                            ],
                        }),
                        Node::Instruction(Instruction {
                            id: "jnz".to_string(),
                            args: vec![Value::Immediate(1)],
                        }),
                    ]);
                }
                Item::Directive(Directive::Static(k, v)) => {
                    if self.statics.contains_key(&k) {
                        bail!("Error: attempted to define {k} twice");
                    }
                    self.statics.insert(k, v);
                }
                Item::Directive(Directive::Dyn(k, v)) => {
                    if self.ram_locations.contains_key(&k) {
                        bail!("Error: attempted to set #dyn {k:#?} twice");
                    }
                    self.ram_locations.insert(k, self.ram_length);
                    self.ram_length += v;
                }
                Item::Directive(Directive::DynOrigin(v)) => {
                    self.ram_origin = v;
                }
                Item::Directive(Directive::Macro(m)) => {
                    if self.macros.contains_key(&m.id) {
                        bail!("Error: attempted to set macro {:#?} twice", m.id);
                    }

                    self.macros.insert(m.id.to_string(), m);
                }
                Item::Directive(Directive::ExplicitBytes(id, bytes)) => {
                    self.tree.push(Node::Explicit(id, bytes));
                }
                Item::Node(n) => self.tree.push(n),
            }
        }

        Ok(())
    }
}
