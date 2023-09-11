use std::iter::Peekable;
use std::vec::IntoIter;

use crate::compiler::{ast::AstNode, tokenizer::Token};

#[macro_use]
mod macros;

mod directive;
mod word;

#[derive(Debug)]
pub(crate) struct Lexer<'s> {
    tokens: Peekable<IntoIter<Token>>,
    file: &'s str,
    pub nodes: Vec<AstNode>,
}

impl<'s> Lexer<'s> {
    pub(crate) fn new(tokens: Vec<Token>, file: &'s str) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            file,
            nodes: vec![],
        }
    }

    pub(crate) fn lex(mut self) -> Result<Self, String> {
        while let Some(token) = self.tokens.next() {
            match token {
                Token::Space | Token::NewLine => continue,
                Token::Directive => self.lex_directive()?,
                Token::Word(word) => self.lex_word(word)?,
                oth => Err(format!("Unexpected symbol: {oth:?}"))?,
            }
        }
        Ok(self)
    }
}
