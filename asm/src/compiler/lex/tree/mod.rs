use std::path::PathBuf;

use crate::compiler::lex::lexable::*;

mod item;
pub use item::*;

use indexmap::IndexMap;

use super::{Directive, Macro, Node};

#[derive(Debug, Default)]
pub struct NodeTree<'buf> {
    pub labels: IndexMap<String, usize>,
    pub statics: IndexMap<&'buf str, usize>,
    pub vars: IndexMap<&'buf str, usize>,
    macros: IndexMap<&'buf str, Macro<'buf>>,
    pub files: Vec<PathBuf>,
    pub mem_root: usize,
    pub boot_to: Option<&'buf str>,
    nodes: Vec<Node<'buf>>,
}

impl<'b> Lexable<'b> for NodeTree<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let mut ctx = NodeTree::default();
        let mut buf = buf;

        loop {
            buf = ignore_whitespace(buf);
            if buf.is_empty() {
                break;
            }
            let (n, b) = Item::lex(buf)?;
            buf = b;
            match n {
                Item::Directive(d) => match d {
                    Directive::Boot(to) => {
                        if ctx.boot_to.is_some() {
                            return Err(LexError::Redefinition("#[boot]".to_string()));
                        }
                        ctx.boot_to = Some(to);
                    }
                    Directive::DynOrigin(org) => {
                        if ctx.mem_root != 0 {
                            return Err(LexError::Redefinition("#[dyn(&)]".to_string()));
                        }
                        ctx.mem_root = org;
                    }
                    Directive::ExplicitBytes(id, explicit) => {
                        ctx.nodes.push(Node::Explicit(id, explicit));
                    }
                    Directive::Macro(m) => {
                        if ctx.macros.contains_key(m.id) {
                            return Err(LexError::Redefinition(m.id.to_string()));
                        }
                        ctx.macros.insert(m.id, m);
                    }
                    Directive::Static(id, val) => {
                        if ctx.statics.contains_key(id) {
                            return Err(LexError::Redefinition(id.to_string()));
                        }
                        ctx.statics.insert(id, val);
                    }
                    Directive::Use(import) => ctx.nodes.push(Node::Import(import)),
                    Directive::Dyn(id, len) => {
                        if ctx.vars.contains_key(id) {
                            return Err(LexError::Redefinition(id.to_string()));
                        }
                        ctx.vars.insert(id, len);
                    }
                },
                Item::Node(n) => ctx.nodes.push(n),
            }
        }

        Ok((ctx, buf))
    }
}
