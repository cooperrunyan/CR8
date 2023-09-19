use crate::compiler::ast::{AddrByte, Instruction, Label, ToValue, Value};
use crate::compiler::ast::{Directive, ToNode};
use crate::op::Operation;
use crate::reg::Register;

use anyhow::{anyhow, bail, Result};
use log::warn;

mod macros;

use super::Lexer;
use super::Token;

impl Lexer {
    pub(crate) fn lex_directive(&mut self) -> Result<()> {
        expect!(self, "Expected '[' after: '#'", match is_brack_open);

        let mut directive = vec![];

        while_next!(self, next, {
            match &next.token {
                Token::Space | Token::NewLine => continue,
                Token::BracketClose => break,
                _ => directive.push(next),
            }
        });

        let mut directive = directive.into_iter();

        match directive.next() {
            None => {
                warn!("Found empty '#[]' statement");
            }
            Some(t) => match &t.token {
                Token::Word(word) => match word.as_str() {
                    "use" => {
                        match next_in!(directive) {
                            t => match &t.token {
                                Token::ParenOpen => {}
                                t => bail!("Unexpected token: {t:#?}"),
                            },
                        }
                        let next = next_in!(directive);
                        match &next.token {
                            Token::Word(w) => match w.as_str() {
                                "std" => {
                                    let mut std_import = format!("${}", w);
                                    while_next_in!(directive, next, {
                                        match &next.token {
                                            Token::ParenClose => {
                                                if directive.next().is_some() {
                                                    bail!("Unexpected after {:#?}", next.token);
                                                }
                                            }
                                            _ => {
                                                std_import.push_str(next.token.to_string().as_str())
                                            }
                                        }
                                    });
                                    self.nodes.push(
                                        Directive::Import(std_import, self.file.clone()).to_node(),
                                    );
                                }
                                _ => bail!("Invalid import to {w:?}"),
                            },
                            Token::String(path) => {
                                self.nodes.push(
                                    Directive::Import(path.to_string(), self.file.clone())
                                        .to_node(),
                                );
                                expect_in!(directive, "Syntax error", match is_paren_close);
                                let next = directive.next();
                                if next.is_some() {
                                    bail!("Unexpected after {:#?}", next.unwrap().token);
                                }
                            }
                            oth => bail!("Unexpected {oth:?}"),
                        }
                    }
                    "macro" => {
                        let n = directive.next();
                        if n.is_some() {
                            bail!("Unexpected: {n:#?}");
                        }
                        self.lex_macro()?
                    }
                    "static" => {
                        expect_in!(directive, "Syntax error", match is_paren_open);
                        let name = expect_in!(directive, "Expected name for static", Word(x));
                        expect_in!(directive, "Expected assignment", match is_colon);
                        let val = expect_in!(directive, "Expected value for static", Number(n));
                        expect_in!(directive, "Expected ')'", match is_paren_close);

                        self.nodes
                            .push(Directive::Define(name, val as u128).to_node());
                    }
                    "boot" => {
                        let n = directive.next();
                        if n.is_some() {
                            bail!("Unexpected: {n:#?}");
                        }
                        ignore!(self, Token::Space | Token::NewLine);
                        let label = expect!(self, "Expected label after #[boot]", Word(x));
                        ignore!(self, Token::Space | Token::NewLine);
                        expect!(self, "Expected label after #[boot]", match is_colon);

                        let nodes = vec![
                            Instruction::Native(
                                Operation::MOV,
                                vec![
                                    Register::L.to_value(),
                                    AddrByte::Low(label.clone()).to_value(),
                                ],
                            )
                            .to_node(),
                            Instruction::Native(
                                Operation::MOV,
                                vec![
                                    Register::H.to_value(),
                                    AddrByte::High(label.clone()).to_value(),
                                ],
                            )
                            .to_node(),
                            Instruction::Native(Operation::JNZ, vec![Value::Immediate(1)])
                                .to_node(),
                        ];
                        self.nodes.push(Directive::Preamble(nodes).to_node());
                        self.nodes.push(Label::Label(label).to_node());
                        return Ok(());
                    }
                    "mem" => {
                        expect_in!(directive, "Expected '('", match is_paren_open);
                        let name = expect_in!(directive, "Expected mem name", Word(x));
                        expect_in!(directive, "Expected ')'", match is_paren_close);
                        let n = directive.next();
                        if n.is_some() {
                            bail!("Unexpected: {n:#?}");
                        }
                        let val = self.lex_mem_definition()?;
                        self.nodes.push(Directive::Rom(name, val).to_node());
                    }
                    "dyn" => {
                        expect_in!(directive, "Expected '('", match is_paren_open);
                        let next = expect_in!(directive, "Expected dyn name", match is_word | is_ampersand);
                        match next {
                            Token::Ampersand => {
                                let from =
                                    expect_in!(directive, "Expected dyn start value", Number(x));
                                self.nodes
                                    .push(Directive::DynamicOrigin(from as u128).to_node());
                            }
                            Token::Word(name) => {
                                expect_in!(directive, "Expected dyn length", match is_equal);
                                let len = expect_in!(directive, "Expected dyn length", Number(x));

                                self.nodes
                                    .push(Directive::Dynamic(name, len as u128).to_node());
                            }
                            t => bail!("Unexpected {t:?}"),
                        }
                        expect_in!(directive, "Expected ')'", match is_paren_close);
                        let n = directive.next();
                        if n.is_some() {
                            bail!("Unexpected: {n:#?}");
                        }
                    }
                    oth => bail!("Unknown directive {oth:?}"),
                },
                _ => bail!("Unexpected {t:#?}"),
            },
        }

        Ok(())
    }

    fn lex_mem_definition(&mut self) -> Result<Vec<u8>> {
        let mut val = vec![];
        let mut open = false;
        while_next!(self, next, {
            match next.token {
                Token::MustacheOpen => {
                    if !open {
                        open = true;
                    } else {
                        bail!("Unexpected '{{'");
                    }
                }
                Token::Number(n) => {
                    val.push(n as u8);
                    if !open {
                        break;
                    }
                    while_peek!(self, next, {
                        match next {
                            Token::Comma => break,
                            Token::MustacheClose => {
                                if !open {
                                    bail!("Unexpected '}}'");
                                }
                                break;
                            }
                            Token::Space | Token::NewLine => {
                                self.tokens.next();
                            }
                            _ => {
                                bail!("Syntax error")
                            }
                        }
                    });
                }
                Token::Comma | Token::NewLine | Token::Space => continue,
                Token::MustacheClose => break,
                _ => bail!("Syntax error"),
            }
        });
        Ok(val)
    }
}
