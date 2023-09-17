use crate::compiler::ast::Directive;
use crate::compiler::ast::Macro;
use crate::compiler::ast::MacroArg;
use crate::compiler::ast::ToNode;

use anyhow::{anyhow, bail, Result};

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
            "origin" => {
                expect!(self, "Syntax error", match is_space);
                let addr = expect!(self, "Expected address after #origin", Number(x));
                self.nodes.push(Directive::Origin(addr as u128).to_node());
            }
            "define" => {
                expect!(self, "Syntax error", match is_space);
                let name = expect!(self, "Expected name for #define statement", Word(x));
                expect!(self, "Syntax error", match is_space);
                let val = expect!(self, "Expected value for #define statement", Number(x));
                self.nodes
                    .push(Directive::Define(name, val as u128).to_node());
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
            "macro" => {
                ignore!(self, Token::NewLine | Token::Space);
                let name = expect!(self, "Expected macro name", Word(x));
                ignore!(self, Token::Space);
                expect!(self, "Expected macro args", match is_brack_open);
                ignore!(self, Token::Space);

                let mut args = vec![];
                while_next!(self, next, {
                    ignore!(self, Token::Space);
                    match next.token {
                        Token::Word(arg) => {
                            if arg.starts_with("ir") {
                                args.push(MacroArg::ImmReg(arg))
                            } else if arg.starts_with('i') {
                                args.push(MacroArg::Immediate(arg))
                            } else if arg.starts_with('r') {
                                args.push(MacroArg::Register(arg))
                            } else if arg.starts_with('a') {
                                args.push(MacroArg::Addr(arg))
                            } else {
                                bail!("Macro arg should start with 'i' 'ir' 'r' or 'a' to signify its type")
                            }
                        }
                        Token::Comma => continue,
                        Token::BracketClose => break,
                        oth => bail!("Unexpected value: {oth:?}"),
                    }
                });

                expect!(self, "Invalid macro syntax", match is_colon);

                let mut body = vec![];

                while_next!(self, next, {
                    match next.token.clone() {
                        Token::NewLine => {
                            match self.tokens.peek().map(|x| x.token.clone()) {
                                Some(Token::NewLine) => break,
                                _ => {}
                            };
                            body.push(next);
                        }
                        _ => body.push(next),
                    }
                });

                let mac_nodes = Lexer::new(body, self.file.clone())
                    .lex()?
                    .nodes
                    .into_iter()
                    .map(|mn| match mn {
                        AstNode::Instruction(inst) => inst,
                        _ => panic!(
                            "Macro body for '{}' should only contain instructions",
                            &name
                        ),
                    })
                    .collect::<Vec<_>>();

                self.nodes.push(Macro::new(name, args, mac_nodes).to_node())
            }
            _ => bail!("Invalid directive: '#{directive}'"),
        };
        Ok(())
    }
}
