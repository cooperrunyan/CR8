use std::fs;

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
    let nodes = match lex(tokens) {
        Ok(n) => n,
        Err(e) => panic!("Error: \n{e}"),
    };

    dbg!(nodes);

    String::new()
}
