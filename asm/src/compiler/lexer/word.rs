use crate::compiler::ast::{Instruction, Label, ToNode, ToValue, Value};
use crate::compiler::tokenizer::Token;
use crate::{op::Operation, reg::Register};

use anyhow::{anyhow, bail, Result};

use super::Lexer;

impl Lexer {
    pub(super) fn lex_word(&mut self, word: String) -> Result<()> {
        match self.tokens.peek().map(|x| x.token.clone()) {
            Some(Token::Colon) => {
                self.nodes.push(Label::from(word).to_node());
                self.tokens.next();
                return Ok(());
            }
            _ => {}
        };
        let inst = word;
        let mut args = vec![];
        while_next!(self, next, {
            match next.token {
                Token::Space => continue,
                Token::Dollar => {
                    let arg = expect!(self, "Expected word after '$'", Word(x));
                    args.push(Value::MacroArg(arg));
                }
                Token::Percent => {
                    let reg = expect!(self, "Unexpected symbol", Word(x));
                    let Ok(reg) = Register::try_from(reg.as_str()) else {
                        bail!("Invalid register: {reg}");
                    };

                    args.push(reg.to_value());
                }
                Token::Number(n) => args.push(n.to_value()),
                Token::BracketOpen => {
                    let mut expr = String::new();
                    while_next!(self, next, {
                        match next.token {
                            Token::BracketClose => break,
                            oth => expr.push_str(&oth.to_string()),
                        }
                    });
                    args.push(Value::Expression(expr))
                }
                Token::Comma => continue,
                Token::NewLine => break,
                oth => bail!("Unexpected value {oth:#?} after {inst:#?}"),
            }
        });

        if let Ok(op) = Operation::try_from(inst.as_str()) {
            self.nodes.push(Instruction::Native(op, args).to_node());
        } else {
            self.nodes.push(Instruction::Macro(inst, args).to_node());
        };

        Ok(())
    }
}
