use std::fs;

use super::Compiler;
use crate::compiler::ast::Directive;
use crate::compiler::lexer::Lexer;
use crate::compiler::{AstNode, STD};

impl Compiler {
    pub(crate) fn resolve_directives(&mut self, nodes: Vec<AstNode>) {
        for node in nodes {
            match node {
                AstNode::Directive(Directive::Import(f)) => {
                    if self.files.contains(&f) {
                        continue;
                    }

                    let file = if f.starts_with("<std>") {
                        if let Some(file) = STD.get(&f) {
                            file.to_string()
                        } else {
                            panic!("Attempted to import non-existent <std> file: {f:#?}")
                        }
                    } else {
                        if let Ok(file) = fs::read_to_string(&f) {
                            file
                        } else {
                            panic!("Unresolved import: {f:#?}")
                        }
                    };

                    let tokens = Compiler::tokenize(&file, &f);
                    let nodes = match Lexer::new(tokens, &f).lex() {
                        Ok(n) => n.nodes,
                        Err(e) => panic!("Error at file: {}\n{}", &f, e),
                    };

                    self.resolve_directives(nodes);

                    self.files.push(f);
                }
                AstNode::Directive(Directive::Define(k, v)) => {
                    if self.statics.contains_key(&k) {
                        panic!("Error: attempted to define {k} twice");
                    }
                    self.statics.insert(k, v);
                }
                AstNode::Directive(Directive::Dynamic(k, v)) => {
                    if self.ram_locations.contains_key(&k) {
                        panic!("Error: attempted to set #dyn {k:#?} twice");
                    }
                    self.ram_locations.insert(k, self.ram_length);
                    self.ram_length += v;
                }
                AstNode::Directive(Directive::Origin(v)) => {
                    self.ram_origin = v as u16;
                }
                AstNode::Directive(Directive::Macro(m)) => {
                    if self.macros.contains_key(&m.name) {
                        panic!("Error: attempted to set macro {:#?} twice", m.name);
                    }

                    self.macros.insert(m.name.clone(), m);
                }
                oth => self.tree.push(oth),
            }
        }
    }
}
