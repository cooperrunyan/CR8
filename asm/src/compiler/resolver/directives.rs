use anyhow::{bail, Result};
use std::fs;
use std::sync::Arc;

use super::Compiler;
use crate::compiler::ast::Directive;
use crate::compiler::lexer::Lexer;
use crate::compiler::{AstNode, STD};

impl Compiler {
    pub(crate) fn resolve_directives(&mut self, nodes: Vec<AstNode>) -> Result<()> {
        for node in nodes {
            match node {
                AstNode::Directive(Directive::Import(f)) => {
                    let mut ex = false;
                    for fexisting in self.files.iter() {
                        if fexisting.as_str() == f.as_str() {
                            ex = true;
                            break;
                        }
                    }
                    if ex {
                        continue;
                    }
                    let file = {
                        let file = if f.starts_with("<std>") {
                            if let Some(file) = STD.get(&f) {
                                file.to_string()
                            } else {
                                bail!("Attempted to import non-existent <std> file: {f:#?}")
                            }
                        } else {
                            if let Ok(file) = fs::read_to_string(&f) {
                                file
                            } else {
                                bail!("Unresolved import: {f:#?}")
                            }
                        };

                        self.files.push(Arc::new(f));
                        file
                    };

                    let nodes = {
                        let f = self.files.last().unwrap();

                        let tokens = Compiler::tokenize(file, f.clone())?;
                        Lexer::new(tokens, f.clone()).lex()?.nodes
                    };

                    self.resolve_directives(nodes)?;
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
                AstNode::Directive(Directive::Origin(v)) => {
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
