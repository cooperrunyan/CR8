use anyhow::{bail, Result};

use super::Compiler;
use crate::compiler::ast::Directive;
use crate::compiler::config::Input;
use crate::compiler::AstNode;

impl Compiler {
    pub(crate) fn resolve_directives(&mut self, nodes: Vec<AstNode>) -> Result<()> {
        for node in nodes {
            match node {
                AstNode::Directive(Directive::Import(f, from)) => {
                    self.push(Input::File(f), from)?;
                }
                AstNode::Directive(Directive::Preamble(block)) => {
                    if self.preamble.is_some() {
                        bail!("Attempted to set init twice");
                    }

                    self.preamble = Some(block);
                }
                AstNode::Directive(Directive::Define(k, v)) => {
                    if self.statics.contains_key(&k) {
                        bail!("Error: attempted to define {k} twice");
                    }
                    self.statics.insert(k.to_string(), v);
                }
                AstNode::Directive(Directive::Dynamic(k, v)) => {
                    if self.ram_locations.contains_key(&k) {
                        bail!("Error: attempted to set #dyn {k:#?} twice");
                    }
                    self.ram_locations.insert(k.to_string(), self.ram_length);
                    self.ram_length += v;
                }
                AstNode::Directive(Directive::DynamicOrigin(v)) => {
                    self.ram_origin = v as u16;
                }
                AstNode::Directive(Directive::Macro(m)) => {
                    if self.macros.contains_key(&m.name) {
                        bail!("Error: attempted to set macro {:#?} twice", m.name);
                    }

                    self.macros.insert(m.name.to_string(), m);
                }
                oth => self.tree.push(oth),
            }
        }

        Ok(())
    }
}
