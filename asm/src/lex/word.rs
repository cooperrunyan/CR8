use std::iter::Peekable;
use std::vec::IntoIter;

use cfg::op::Operation;
use cfg::reg::Register;

use crate::ast::{Addr, AstNode, Instruction, Label, ToNode, ToValue};
use crate::token::Token;

pub fn lex_word(
    word: String,
    tokens: &mut Peekable<IntoIter<Token>>,
    nodes: &mut Vec<AstNode>,
) -> Result<(), String> {
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
            Token::Percent => {
                let reg = next!(
                    tokens,
                    Word(x),
                    "Expected word after '%' to specify register"
                );
                args.push(Register::try_from(reg)?.to_value());
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
                        oth => err!("Todo expression support for: {oth:?}")?,
                    }
                }
            }
            Token::Comma => continue,
            Token::NewLine => break,
            oth => err!("Unexpected value: {oth:#?} after {inst:#?}")?,
        }
    }

    if let Ok(op) = Operation::try_from(inst.as_str()) {
        nodes.push(Instruction::Native(op, args).to_node());
    } else {
        nodes.push(Instruction::Macro(inst, args).to_node());
    };

    Ok(())
}
