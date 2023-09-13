use crate::compiler::ast::{Ident, Instruction, Label, ToNode, ToValue, Value};
use crate::compiler::tokenizer::Token;
use crate::{op::Operation, reg::Register};

use super::Lexer;

impl<'s> Lexer<'s> {
    pub(super) fn lex_word(&mut self, word: String) -> Result<(), String> {
        defnext!(self, word, Word(x));

        if self.tokens.peek() == Some(&Token::Colon) {
            self.nodes.push(Label::from(word).to_node());
            self.tokens.next();
            return Ok(());
        }
        let inst = word;
        let mut args = vec![];
        while let Some(next) = self.tokens.next() {
            match next {
                Token::Space => continue,
                Token::Dollar => {
                    let arg = word!("Expected word after '$'");
                    args.push(Ident::MacroArg(arg).to_value());
                }
                Token::Ampersand => {
                    let stat = word!("Expected static after '&'");
                    args.push(Ident::Static(stat).to_value());
                }
                Token::Percent => {
                    let reg = word!("Unexpected symbol");
                    let Ok(reg) = Register::try_from(reg.as_str()) else {
                        return err!("Invalid register: {reg}");
                    };

                    args.push(reg.to_value());
                }
                Token::Number(n) => args.push(n.to_value()),
                Token::BracketOpen => {
                    let mut expr = String::new();
                    while let Some(next) = self.tokens.next() {
                        match next {
                            Token::BracketClose => break,
                            oth => expr.push_str(&oth.to_string()),
                        }
                    }
                    args.push(Value::Expression(expr))
                }
                Token::Comma => continue,
                Token::NewLine => break,
                oth => err!("Unexpected value {oth:#?} after {inst:#?}")?,
            }
        }

        if let Ok(op) = Operation::try_from(inst.as_str()) {
            self.nodes.push(Instruction::Native(op, args).to_node());
        } else {
            self.nodes.push(Instruction::Macro(inst, args).to_node());
        };

        Ok(())
    }
}
