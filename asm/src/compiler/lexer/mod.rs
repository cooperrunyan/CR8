use anyhow::{bail, Context, Result};
use std::iter::Peekable;
use std::path::PathBuf;
use std::sync::Arc;
use std::vec::IntoIter;

use crate::compiler::{ast::AstNode, tokenizer::Token};

use super::tokenizer::TokenMeta;

#[macro_use]
mod macros;

mod directive;
mod word;

#[derive(Debug)]
pub(crate) struct Lexer {
    tokens: Peekable<IntoIter<TokenMeta>>,
    file: Arc<PathBuf>,
    pub nodes: Vec<AstNode>,
}

impl Lexer {
    pub(crate) fn new(tokens: Vec<TokenMeta>, file: Arc<PathBuf>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            file,
            nodes: vec![],
        }
    }

    pub(crate) fn lex(mut self) -> Result<Self> {
        while let Some(token) = self.tokens.next() {
            match token.token {
                Token::Space | Token::NewLine => continue,
                Token::Directive => self.lex_directive().context(format!(
                    "File {:?}:{}:{}",
                    token.path,
                    token.line + 1,
                    token.col + 1
                ))?,
                Token::Word(word) => self.lex_word(word).context(format!(
                    "File {:?}:{}:{}",
                    token.path,
                    token.line + 1,
                    token.col + 1
                ))?,
                oth => bail!("Unexpected symbol: {oth:?}"),
            }
        }
        Ok(self)
    }
}
