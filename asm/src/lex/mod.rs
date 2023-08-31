use std::path::PathBuf;

use crate::{ast::AstNode, token::Token};

use self::{directive::lex_directive, word::lex_word};

#[macro_use]
mod macros;
mod directive;
mod word;

pub struct LexError {
    pub msg: String,
    pub line: u128,
    pub file: PathBuf,
}

#[macro_export]
macro_rules! err {
    ($line:expr, $file:expr, $msg:expr $(, $args:expr)*) => {
        Err(LexError::new($line, $file, format!($msg, $( $args )*)))
    };
}

impl LexError {
    pub fn new(line: &u128, file: &PathBuf, msg: String) -> Self {
        Self {
            msg,
            line: line.to_owned(),
            file: file.to_owned(),
        }
    }
}

pub fn lex(tokens: Vec<Token>, file: &PathBuf) -> Result<Vec<AstNode>, LexError> {
    let mut nodes = vec![];
    let mut tokens = tokens.into_iter().peekable();

    let mut line = 1;

    while let Some(token) = tokens.next() {
        match token {
            Token::Space => continue,
            Token::NewLine => line += 1,
            Token::Directive => lex_directive(file, &mut line, &mut tokens, &mut nodes)?,
            Token::Word(word) => lex_word(file, word, &mut line, &mut tokens, &mut nodes)?,
            oth => err!(&line, &file, "Unexpected symbol: {oth:?}")?,
        }
    }

    Ok(nodes)
}
