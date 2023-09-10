use std::fs;

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod op;
pub mod mem;
pub mod reg;

mod ast;
mod compiler;
mod config;
mod expr;
mod lex;
mod token;

use compiler::Compiler;
pub use config::Config;

use crate::ast::Ast;

use phf::phf_map;

static STD: phf::Map<&'static str, &'static str> = phf_map! {
    "<std>/arch.asm" => include_str!("./std/arch.asm"),
    "<std>/macros.asm" => include_str!("./std/macros.asm"),
    "<std>/math.asm" => include_str!("./std/math.asm"),
};

pub fn compile(cfg: Config) -> Vec<u8> {
    let (file, source) = if cfg.literal.is_empty() {
        let Ok(source) = fs::read_to_string(&cfg.input) else {
            panic!("Failed to read {:#?}", cfg.input)
        };
        (cfg.input, source)
    } else {
        ("anon".to_string(), cfg.literal)
    };

    let ast = Ast::start(source, file);

    let mut compiler = Compiler::from(ast);
    compiler.strip_labels();
    compiler.compile();

    compiler.bin
}
