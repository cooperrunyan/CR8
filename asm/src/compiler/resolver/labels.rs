use crate::compiler::lex::{Instruction, Node};
use crate::op::{Operation, OperationArgAmt};

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

impl Instruction {
    pub fn size(&self) -> Result<usize> {
        let op = match Operation::try_from(self.id.as_str()) {
            Ok(o) => o,
            Err(_) => bail!("Cannot determine size of {:#?}", self.id),
        };

        op.size(OperationArgAmt::from_args(&self.args)?)
    }
}
