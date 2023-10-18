use anyhow::{bail, Result};
use indexmap::IndexMap;

use crate::compiler::lex::{Expr, ExprOperation, Instruction, MacroCaptureArgType, Node, Value};
use crate::op::Operation;

use super::Compiler;

impl Compiler {
    pub(crate) fn resolve_macros(&mut self) -> Result<()> {
        let mut new_tree = vec![];

        let mut tree = vec![];
        tree.append(&mut self.tree);

        for node in tree {
            let mut stripped = self.fill_macro(node)?;
            new_tree.append(&mut stripped);
        }

        self.tree = new_tree;

        Ok(())
    }

    fn fill_macro(&self, node: Node) -> Result<Vec<Node>> {
        use MacroCaptureArgType as MA;
        use Value as V;

        let mut tree = vec![];

        match node {
            Node::Instruction(inst) => {
                let mac = match self.macros.get(&inst.id) {
                    Some(m) => m,
                    None => {
                        match Operation::try_from(inst.id.as_str()) {
                            Ok(_) => {}
                            Err(_) => bail!("Macro {:#?} not defined", inst.id),
                        };

                        return Ok(vec![Node::Instruction(inst)]);
                    }
                };

                let mut captured_args: IndexMap<String, Value> = IndexMap::new();

                for capturer in mac.captures.iter() {
                    if capturer.args.len() != inst.args.len() {
                        continue;
                    }

                    let mut valid = true;

                    macro_rules! invalid {
                        () => {{
                            valid = false;
                            break;
                        }};
                    }

                    macro_rules! insert {
                        ($n:expr, $t:ident($v:expr)) => {{
                            captured_args.insert($n.to_string(), Value::$t($v.clone()));
                        }};
                    }

                    macro_rules! insert_addr {
                        ($n:expr, $f:expr, $r:expr) => {{
                            captured_args.insert(
                                format!("{}.l", $n),
                                Value::Expr(Expr::Expr {
                                    lhs: Box::new($r),
                                    op: ExprOperation::And,
                                    rhs: Box::new(Expr::Literal(0xFF)),
                                }),
                            );
                            captured_args.insert(
                                format!("{}.h", $n),
                                Value::Expr(Expr::Expr {
                                    lhs: Box::new($r),
                                    op: ExprOperation::Rsh,
                                    rhs: Box::new(Expr::Literal(8)),
                                }),
                            );
                            captured_args.insert($n.to_string(), Value::Expr($r));
                        }};
                    }

                    for (i, capture_arg) in capturer.args.iter().enumerate() {
                        let current = inst.args.get(i).unwrap();
                        let name = &capture_arg.id;
                        match &capture_arg.ty {
                            MA::Literal => match current {
                                V::Literal(v) => insert!(name, Literal(v)),
                                V::MacroVariable(id) => insert!(name, MacroVariable(id)),
                                V::Expr(e) => insert!(name, Expr(e)),
                                _ => invalid!(),
                            },
                            MA::Register => match current {
                                V::Register(r) => insert!(name, Register(r)),
                                _ => invalid!(),
                            },
                            MA::LiteralOrRegister => match current {
                                V::Literal(v) => insert!(name, Literal(v)),
                                V::Register(r) => insert!(name, Register(r)),
                                V::MacroVariable(id) => insert!(name, MacroVariable(id)),
                                V::Expr(e) => insert!(name, Expr(e)),
                            },
                            MA::Expr => match current {
                                V::Expr(e) => {
                                    insert_addr!(name, V::Expr(e.clone()), e.clone());
                                }
                                V::Literal(v) => {
                                    insert_addr!(
                                        name,
                                        V::Expr(Expr::Literal(*v)),
                                        Expr::Literal(*v)
                                    );
                                }
                                _ => invalid!(),
                            },
                        }
                    }

                    if !valid {
                        continue;
                    }

                    for instruction in capturer.content.iter() {
                        let mut new_args: Vec<Value> = vec![];

                        for arg in instruction.args.iter() {
                            match arg {
                                V::MacroVariable(ma) => {
                                    let Some(val) = captured_args.get(ma) else {
                                        panic!(
                                            "Attempted to use undefined macro arg {:#?} at {:#?}",
                                            ma, inst.id
                                        );
                                    };
                                    new_args.push(val.to_owned());
                                }
                                oth => new_args.push(oth.clone()),
                            }
                        }

                        let mut nodes = self.fill_macro(Node::Instruction(Instruction {
                            id: instruction.id.clone(),
                            args: new_args,
                        }))?;

                        tree.append(&mut nodes);
                    }
                    return Ok(tree);
                }

                match Operation::try_from(inst.id.as_str()) {
                    Ok(_) => {}
                    Err(_) => bail!(
                        "Could not find matching macro or instruction for {:#?}",
                        inst.id
                    ),
                };

                return Ok(vec![Node::Instruction(inst)]);
            }
            _ => tree.push(node),
        };

        Ok(tree)
    }
}
