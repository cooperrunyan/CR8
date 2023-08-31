use std::iter::Peekable;
use std::vec::IntoIter;

use cfg::op::Operation;
use cfg::reg::Register;

use crate::ast::{Addr, AstNode, Ident, Instruction, Label, ToNode, ToValue};
use crate::err;
use crate::token::Token;

use super::LexError;

pub fn lex_word<'s>(
    file: &'s str,
    word: String,
    line: &mut u128,
    tokens: &mut Peekable<IntoIter<Token>>,
    nodes: &mut Vec<AstNode>,
) -> Result<(), LexError> {
    if tokens.peek() == Some(&Token::Colon) {
        nodes.push(Label::from(word).to_node());
        tokens.next();
        return Ok(());
    }
    let inst = word;
    let mut args = vec![];
    while let Some(next) = tokens.next() {
        match next {
            Token::Space => continue,
            Token::Dollar => {
                let Some(arg) = next!(tokens, Word(x)) else {
                    return err!(line, file, "Expected word after '$'");
                };
                args.push(Ident::MacroArg(arg).to_value());
            }
            Token::Ampersand => {
                let Some(stat) = next!(tokens, Word(x)) else {
                    return err!(line, file, "Expected static after '&'");
                };
                args.push(Ident::Static(stat).to_value());
            }
            Token::Percent => {
                let Some(reg) = next!(tokens, Word(x)) else {
                    return err!(line, file, "Unexpected symbol");
                };
                let Ok(reg) = Register::try_from(reg) else {
                    return err!(line, file, "Invalid register");
                };

                args.push(reg.to_value());
            }
            Token::Number(n) => args.push(n.to_value()),
            Token::BracketOpen => {
                while let Some(next) = tokens.next() {
                    match next {
                        Token::Word(w) => {
                            args.push(Addr::LowByte(w.clone()).to_value());
                            args.push(Addr::HighByte(w).to_value());
                        }
                        Token::BracketClose => break,
                        Token::Space => continue,
                        oth => err!(line, file, "Todo: expression value for {oth:#?}")?,
                    }
                }
            }
            Token::Comma => continue,
            Token::NewLine => break,
            oth => err!(line, file, "Unexpected value {oth:#?} after {inst:#?}")?,
        }
    }

    if let Ok(op) = Operation::try_from(inst.as_str()) {
        nodes.push(Instruction::Native(op, args).to_node());
    } else {
        nodes.push(Instruction::Macro(inst, args).to_node());
    };

    Ok(())
}
