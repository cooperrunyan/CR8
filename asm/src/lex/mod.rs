use crate::{ast::AstNode, token::Token};

use self::{directive::lex_directive, word::lex_word};

#[macro_use]
mod macros;
mod directive;
mod word;

pub fn lex(tokens: Vec<Token>) -> Result<Vec<AstNode>, String> {
    let mut nodes = vec![];
    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        match token {
            Token::Space | Token::NewLine => continue,
            Token::Directive => lex_directive(&mut tokens, &mut nodes)?,
            Token::Word(word) => lex_word(word, &mut tokens, &mut nodes)?,
            oth => err!("Unexpected symbol: {oth:#?}")?,
        }
    }

    Ok(nodes)
}
