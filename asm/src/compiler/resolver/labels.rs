use crate::compiler::lex::{Instruction, Node, Value};
use crate::op::Operation;

use anyhow::{bail, Result};

use super::Compiler;

impl Compiler {
    pub(crate) fn resolve_labels(&mut self) -> Result<()> {
        for node in self.tree.iter() {
            match node {
                Node::Label(ln) => {
                    if ln.starts_with('.') {
                        self.labels
                            .insert(format!("{}{ln}", self.last_label), self.pc);
                    } else {
                        self.last_label = ln.to_string();
                        self.labels.insert(ln.to_string(), self.pc);
                    }
                }

                Node::Instruction(inst) => {
                    let size = match inst.size() {
                        Ok(sz) => sz,
                        Err(e) => bail!("Argument error for {inst:#?}: {e:#?}. At {node:#?}"),
                    };
                    self.pc += size as usize;
                }
                Node::Explicit(name, val) => {
                    let len = val.0.len();
                    self.labels.insert(name.to_string(), self.pc);
                    self.pc += len;
                }
                oth => bail!("Unexpected {oth:#?}. At {node:#?}"),
            }
        }

        Ok(())
    }
}

impl Operation {
    pub(crate) fn size(&self, _args: &Vec<Value>) -> Result<usize> {
        use Operation::*;
        let mut args = vec![];
        for arg in _args {
            args.push(arg)
        }

        macro_rules! none {
            ($a:expr, $n:expr) => {
                if $a.next().is_none() {
                    Ok($n as usize)
                } else {
                    dbg!(&$a);
                    bail!("Too many arguments")
                }
            };
        }
        let mut args = args.into_iter();

        match self {
            LW | SW => {
                let mut args = match self {
                    LW => args.collect::<Vec<_>>().into_iter(),
                    SW => args.rev().collect::<Vec<_>>().into_iter(),
                    _ => bail!(""),
                };

                let Some(Value::Register(_)) = args.next() else {
                    bail!("Expected the first argument of LW to be a register");
                };
                match args.next() {
                    None => Ok(1),
                    Some(Value::Immediate(_)) => match args.next() {
                        Some(Value::Immediate(_)) => none!(args, 3),
                        _ => bail!("Expected another address byte for LW"),
                    },
                    Some(Value::Expr(_)) => none!(args, 3),
                    oth => bail!("Unexpected additional argument: {oth:#?}"),
                }
            }
            PUSH => match args.next() {
                Some(Value::Immediate(_) | Value::Expr(_)) => none!(args, 2),
                Some(Value::Register(_)) => none!(args, 1),
                _ => bail!("Expected register or immediate as first argument"),
            },
            JNZ => {
                let len = match args.next() {
                    Some(Value::Immediate(_) | Value::Expr(_)) => 2,
                    Some(Value::Register(_)) => 1,
                    _ => bail!("Expected register or immediate as first argument"),
                };
                none!(args, len)
            }
            POP => {
                let Some(Value::Register(_)) = args.next() else {
                    bail!("Expected register as only argument");
                };
                none!(args, 1)
            }
            MB => {
                let Some(Value::Immediate(_)) = args.next() else {
                    bail!("Expected immediate as only argument");
                };
                none!(args, 2)
            }
            OUT | IN | MOV | CMP | ADC | SBB | OR | NOR | AND => {
                let mut args = match self {
                    OUT => args.rev().collect::<Vec<_>>().into_iter(),
                    _ => args.collect::<Vec<_>>().into_iter(),
                };

                let Some(Value::Register(_)) = args.next() else {
                    bail!("Expected register argument");
                };
                match args.next() {
                    None => bail!("Expected another argument"),
                    Some(_) => none!(args, 2),
                }
            }
        }
    }
}

impl Instruction {
    pub fn size(&self) -> Result<usize> {
        let op = match Operation::try_from(self.id.as_str()) {
            Ok(o) => o,
            Err(_) => bail!("Cannot determine size of {:#?}", self.id),
        };

        op.size(&self.args)
    }
}
