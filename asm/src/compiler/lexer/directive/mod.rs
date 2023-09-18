use crate::compiler::ast::{Directive, ToNode};

use anyhow::{anyhow, bail, Result};

mod macros;

use super::AstNode;
use super::Lexer;
use super::Token;

impl Lexer {
    pub(crate) fn lex_directive(&mut self) -> Result<()> {
        let directive = expect!(self, "Expected directive after: '#'", Word(x));
        match directive.as_str() {
            "include" => {
                expect!(self, "Syntax error", match is_space);
                let path = expect!(self, "Expected path after #include statement", String(x));
                self.nodes
                    .push(Directive::Import(path, self.file.clone()).to_node());
            }
            "dynorg" => {
                expect!(self, "Syntax error", match is_space);
                let addr = expect!(self, "Expected address after #dynorg", Number(x));
                self.nodes
                    .push(Directive::DynamicOrigin(addr as u128).to_node());
            }
            "define" => {
                expect!(self, "Syntax error", match is_space);
                let name = expect!(self, "Expected name for #define statement", Word(x));
                expect!(self, "Syntax error", match is_space);
                let val = expect!(self, "Expected value for #define statement", Number(x));
                self.nodes
                    .push(Directive::Define(name, val as u128).to_node());
            }
            "init" => {
                let mut init = vec![];
                let mut open = false;
                while_next!(self, next, {
                    match &next.token {
                        Token::MustacheOpen => open = true,
                        Token::MustacheClose => {
                            if open {
                                break;
                            }
                        }
                        Token::NewLine => {
                            if !open {
                                break;
                            }
                            init.push(next);
                        }
                        _ => init.push(next),
                    };
                });

                let init_nodes = Lexer::new(init, self.file.clone()).lex()?.nodes;
                self.nodes
                    .push(AstNode::Directive(Directive::Preamble(init_nodes)))
            }
            "dyn" | "mem" => {
                expect!(self, "Syntax error", match is_space);

                let next =
                    expect!(self, "Expected length after '#{directive:?}'", match is_num | is_word);

                let len = match next {
                    Token::Word(w) => match w.as_str() {
                        "byte" => 1,
                        "word" => 2,
                        _ => bail!("Expected length after '#{directive:?}'"),
                    },
                    Token::Number(l) => l,
                    _ => bail!("Expected length after '#{directive:?}'"),
                };

                expect!(self, "Syntax error", match is_space);

                let name = expect!(self, "Expected name after '#{directive:?}'", Word(x));

                if directive == "mem" {
                    expect!(self, "Syntax error", match is_space);

                    let mut val = vec![];
                    if len == 1 {
                        let v = expect!(self, "Expected value after #mem assignment", Number(x));
                        val.push(v as u8);
                    } else if len == 2 {
                        let v = expect!(self, "Expected value after #mem assignment", Number(x));
                        val.push(v as u8);
                        val.push((v >> 8) as u8);
                    } else {
                        expect!(self, "Expected '[0, 0, ...]' for #mem assignments longer than 2", match is_brack_open);
                        while_next!(self, next_num, {
                            match next_num.token {
                                Token::Number(n) => {
                                    val.push(n as u8);
                                    while_peek!(self, next_num, {
                                        match next_num {
                                            Token::Comma => break,
                                            Token::BracketClose => break,
                                            Token::Space | Token::NewLine => {
                                                self.tokens.next();
                                            }
                                            other => {
                                                bail!("Expected [0, 0, 0, ...], got: {other:?}")
                                            }
                                        }
                                    });
                                }
                                Token::Comma | Token::NewLine | Token::Space => continue,
                                Token::BracketClose => break,
                                other => bail!("Expected [0, 0, 0, ...], got: {other:?}"),
                            }
                        });
                    }

                    if val.len() != len as usize {
                        bail!(
                            "Expected {name} to be {len} bytes long, got {} bytes",
                            val.len()
                        );
                    }

                    self.nodes.push(Directive::Rom(name, val).to_node())
                } else {
                    self.nodes
                        .push(Directive::Dynamic(name, len as u128).to_node())
                }
            }
            "macro" => self.lex_macro()?,
            _ => bail!("Invalid directive: '#{directive}'"),
        };
        Ok(())
    }
}
