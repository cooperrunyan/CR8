use std::fs;

use ast::Ast;

mod ast;
mod config;
mod lex;
mod token;

pub use config::Config;
use lex::lex;
use token::tokenize;

pub fn compile(cfg: Config) -> String {
    let source = if cfg.literal.is_empty() {
        let Ok(source) = fs::read_to_string(&cfg.input) else {
            panic!("Failed to read {:#?}", cfg.input)
        };
        source
    } else {
        cfg.literal
    };

    let tokens = tokenize(&source);
    let nodes = lex(tokens);

    dbg!(nodes);

    String::new()
}
