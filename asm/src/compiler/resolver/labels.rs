use crate::compiler::ast::{AstNode, Directive, Instruction, Label, Value};
use crate::op::Operation;

use super::Compiler;

impl Compiler {
    pub(crate) fn resolve_labels(&mut self) {
        for node in self.tree.iter() {
            match node {
                AstNode::Label(Label::Label(ln)) => {
                    self.last_label = ln.to_string();
                    self.labels.insert(ln.to_string(), self.pc);
                }
                AstNode::Label(Label::SubLabel(sub)) => {
                    self.labels
                        .insert(format!("{}{sub}", self.last_label), self.pc);
                }
                AstNode::Instruction(Instruction::Native(op, args)) => {
                    let size = match op.size(args) {
                        Ok(sz) => sz,
                        Err(e) => panic!("Argument error for {op:#?}: {e:#?}. At {node:#?}"),
                    };
                    self.pc += size as usize;
                }
                AstNode::Directive(Directive::Rom(name, val)) => {
                    let len = val.len();
                    self.labels.insert(name.to_string(), self.pc);
                    self.pc += len;
                }
                oth => panic!("Unexpected {oth:#?}. At {node:#?}"),
            }
        }
    }
}

impl Operation {
    pub fn size(&self, _args: &Vec<Value>) -> Result<u8, String> {
        use Operation::*;
        let mut args = vec![];
        for arg in _args {
            args.push(arg)
        }

        macro_rules! none {
            ($a:expr, $n:expr) => {
                if $a.next().is_none() {
                    Ok($n as u8)
                } else {
                    dbg!(&$a);
                    Err("Too many arguments".to_string())
                }
            };
        }
        let mut args = args.into_iter();

        match self {
            LW | SW => {
                let mut args = match self {
                    LW => args.collect::<Vec<_>>().into_iter(),
                    SW => args.rev().collect::<Vec<_>>().into_iter(),
                    _ => panic!(),
                };

                let Some(Value::Register(_)) = args.next() else {
                    return Err("Expected the first argument of LW to be a register".to_string());
                };
                match args.next() {
                    None => return Ok(1),
                    Some(Value::AddrByte(_) | Value::Immediate(_)) => match args.next() {
                        Some(Value::AddrByte(_) | Value::Immediate(_)) => return none!(args, 3),
                        _ => return Err("Expected another address byte for LW".to_string()),
                    },
                    Some(Value::Expression(_) | Value::Ident(_)) => return none!(args, 3),
                    oth => return Err(format!("Unexpected additional argument: {oth:#?}")),
                }
            }
            PUSH => {
                match args.next() {
                    Some(Value::Immediate(_) | Value::Expression(_)) => return none!(args, 2),
                    Some(Value::Register(_)) => return none!(args, 1),
                    _ => return Err("Expected register or immediate as first argument".to_string()),
                };
            }
            JNZ => {
                let len = match args.next() {
                    Some(Value::Immediate(_) | Value::Expression(_)) => 2,
                    Some(Value::Register(_)) => 1,
                    _ => return Err("Expected register or immediate as first argument".to_string()),
                };
                return none!(args, len);
            }
            POP => {
                let Some(Value::Register(_)) = args.next() else {
                    return Err("Expected register as only argument".to_string());
                };
                return none!(args, 1);
            }
            MB => {
                let Some(Value::Immediate(_)) = args.next() else {
                    return Err("Expected immediate as only argument".to_string());
                };
                return none!(args, 2);
            }
            OUT | IN | MOV | CMP | ADC | SBB | OR | NOR | AND => {
                let mut args = match self {
                    OUT => args.rev().collect::<Vec<_>>().into_iter(),
                    _ => args.collect::<Vec<_>>().into_iter(),
                };

                let Some(Value::Register(_)) = args.next() else {
                    return Err("Expected register argument".to_string());
                };
                match args.next() {
                    None => return Err("Expected another argument".to_string()),
                    Some(_) => return none!(args, 2),
                }
            }
        }
    }
}
