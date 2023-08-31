use std::fs;

mod ast;
mod config;
mod lex;
mod token;

use ast::{AstNode, Directive};
pub use config::Config;
use lex::lex;
use token::tokenize;

use crate::ast::Ast;

use phf::phf_map;

static STD: phf::Map<&'static str, &'static str> = phf_map! {
    "<std>/arch.asm" => include_str!("./std/arch.asm"),
    "<std>/macros.asm" => include_str!("./std/macros.asm"),
    "<std>/math.asm" => include_str!("./std/math.asm"),
};

pub fn compile(cfg: Config) -> String {
    let (file, source) = if cfg.literal.is_empty() {
        let Ok(source) = fs::read_to_string(&cfg.input) else {
            panic!("Failed to read {:#?}", cfg.input)
        };
        (cfg.input, source)
    } else {
        ("anon".to_string(), cfg.literal)
    };

    let tokens = tokenize(&source, &file);
    let nodes = match lex(tokens, &file) {
        Ok(n) => n,
        Err(e) => panic!("Error at file: {:?}:{}:\n\n{}", e.file, e.line, e.msg),
    };

    let mut ctx = Ast::default();

    strip(&mut ctx, nodes);

    dbg!(ctx.tree);

    String::new()
}

fn strip(ctx: &mut Ast, nodes: Vec<AstNode>) {
    for node in nodes {
        match node {
            AstNode::Directive(Directive::Import(f)) => {
                if ctx.files.contains(&f) {
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

                let tokens = tokenize(&file, &f);
                let nodes = match lex(tokens, &f) {
                    Ok(n) => n,
                    Err(e) => panic!("Error at file: {}:{}\n\n{}", e.file, e.line, e.msg),
                };

                ctx.files.push(f);

                strip(ctx, nodes);
            }
            AstNode::Directive(Directive::Define(k, v)) => {
                if ctx.statics.contains_key(&k) {
                    panic!("Error: attempted to define {k} twice");
                }
                ctx.statics.insert(k, v);
            }
            AstNode::Directive(Directive::Dynamic(k, v)) => {
                if ctx.ram_locations.contains_key(&k) {
                    panic!("Error: attempted to set #dyn {k:#?} twice");
                }
                ctx.ram_locations.insert(k, ctx.ram_length);
                ctx.ram_length += v;
            }
            AstNode::Directive(Directive::Origin(v)) => {
                ctx.ram_origin = v as u16;
            }
            AstNode::Directive(Directive::Macro(m)) => {
                if ctx.macros.contains_key(&m.name) {
                    panic!("Error: attempted to set macro {:#?} twice", m.name);
                }

                ctx.macros.insert(m.name.clone(), m);
            }
            oth => ctx.tree.push(oth),
        }
    }
}
