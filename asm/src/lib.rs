use std::fs;
use std::path::PathBuf;

mod ast;
mod config;
mod lex;
mod token;

pub use config::Config;
use lex::lex;
use token::tokenize;

pub fn compile(cfg: Config) -> String {
    let (file, source) = if cfg.literal.is_empty() {
        let Ok(source) = fs::read_to_string(&cfg.input) else {
            panic!("Failed to read {:#?}", cfg.input)
        };
        (cfg.input, source)
    } else {
        (PathBuf::from("anon"), cfg.literal)
    };

    let tokens = tokenize(&source, &file);
    let nodes = match lex(tokens, &file) {
        Ok(n) => n,
        Err(e) => panic!("Error at file: {:?}:{}:\n\n{}", e.file, e.line, e.msg),
    };

    dbg!(nodes);

    String::new()
}
