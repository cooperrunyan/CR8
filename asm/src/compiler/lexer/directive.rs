use crate::compiler::ast::Directive;
use crate::compiler::ast::Macro;
use crate::compiler::ast::MacroArg;
use crate::compiler::ast::ToNode;

use super::AstNode;
use super::Lexer;
use super::Token;

impl<'s> Lexer<'s> {
    pub(crate) fn lex_directive(&mut self) -> Result<(), String> {
        defnext!(self, word, Word(x));
        defnext!(self, num, Number(x));
        defnext!(self, stri, String(x));
        defnext!(self, brackopen, BracketOpen);
        defnext!(self, space, Space);
        defnext!(self, colon, Colon);

        let directive = word!("Expected directive after: '#'");
        match directive.as_str() {
            "include" => {
                space!("Syntax error");
                let path = stri!("Expected path after #include statement");
                self.nodes.push(Directive::Import(path).to_node());
            }
            "origin" => {
                space!("Syntax error");
                let addr = num!("Expected address after #origin");
                self.nodes.push(Directive::Origin(addr as u128).to_node());
            }
            "define" => {
                space!("Syntax error");
                let name = word!("Expected name for #define statement");
                space!("Syntax error");
                let val = num!("Expected value for #define statement");
                self.nodes
                    .push(Directive::Define(name, val as u128).to_node());
            }
            "dyn" | "mem" => {
                space!("Syntax error");

                let len = match self.tokens.next() {
                    Some(Token::Word(s)) => match s.as_str() {
                        "byte" => 1,
                        "word" => 2,
                        t => err!("Invalid type for #{directive:?} definition: {t}")?,
                    },
                    Some(Token::Number(l)) => l,
                    _ => err!("Expected length after '#{directive:?}'")?,
                };

                space!("Syntax error");

                let name = word!("Expected name after '#{directive:?}'");

                if directive == "mem" {
                    space!("Syntax error");

                    let mut val = vec![];
                    if len == 1 {
                        let v = num!("Expected value after #mem assignment");
                        val.push(v as u8);
                    } else if len == 2 {
                        let v = num!("Expected value after #mem assignment");
                        val.push(v as u8);
                        val.push((v >> 8) as u8);
                    } else {
                        brackopen!("Expected '[0, 0, ...]' for #mem assignments longer than 2");
                        while let Some(next_num) = self.tokens.next() {
                            match next_num {
                                Token::Number(n) => {
                                    val.push(n as u8);
                                    while let Some(next_num) = self.tokens.peek() {
                                        match next_num {
                                            Token::Comma => break,
                                            Token::BracketClose => break,
                                            Token::Space | Token::NewLine => {
                                                self.tokens.next();
                                            }
                                            other => {
                                                err!("Expected [0, 0, 0, ...], got: {other:?}")?
                                            }
                                        }
                                    }
                                }
                                Token::Comma | Token::NewLine | Token::Space => continue,
                                Token::BracketClose => break,
                                other => err!("Expected [0, 0, 0, ...], got: {other:?}")?,
                            }
                        }
                    }

                    if val.len() != len as usize {
                        err!(
                            "Expected {name} to be {len} bytes long, got {} bytes",
                            val.len()
                        )?
                    }

                    self.nodes.push(Directive::Rom(name, val).to_node())
                } else {
                    self.nodes
                        .push(Directive::Dynamic(name, len as u128).to_node())
                }
            }
            "macro" => {
                ignore_line_space!(self.tokens);
                let name = word!("Expected macro name");
                ignore_space!(self.tokens);
                brackopen!("Expected macro args");
                ignore_space!(self.tokens);

                let mut args = vec![];
                while let Some(next) = self.tokens.next() {
                    ignore_space!(self.tokens);
                    match next {
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
                                err!("Macro arg should start with 'i' 'ir' 'r' or 'a' to signify its type")?
                            }
                        }
                        Token::Comma => continue,
                        Token::BracketClose => break,
                        oth => err!("Unexpected value: {oth:?}")?,
                    }
                }

                colon!("Invalid macro syntax");

                let mut body = vec![];

                while let Some(next) = self.tokens.next() {
                    match next {
                        Token::NewLine => {
                            if self.tokens.peek() == Some(&Token::NewLine) {
                                break;
                            }
                            body.push(Token::NewLine);
                        }
                        oth => body.push(oth),
                    }
                }

                let mac_nodes = Lexer::new(body, self.file)
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
            _ => err!("Invalid directive: '#{directive}'")?,
        };
        Ok(())
    }
}
