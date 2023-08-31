use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::{AstNode, Directive, Macro, MacroArg, ToNode};
use crate::lex::lex;
use crate::token::Token;

pub fn lex_directive(
    tokens: &mut Peekable<IntoIter<Token>>,
    nodes: &mut Vec<AstNode>,
) -> Result<(), String> {
    let directive = next!(tokens, Word(v), "Expected directive after: '#'");
    match directive.as_str() {
        "include" => {
            next!(tokens, Space);
            let path = next!(tokens, String(x), "path");
            nodes.push(Directive::Import(path).to_node());
        }
        "origin" => {
            next!(tokens, Space);
            let addr = next!(tokens, Number(x), "Expected address after #origin");
            nodes.push(Directive::Origin(addr as u128).to_node());
        }
        "define" => {
            next!(tokens, Space);
            let name = next!(tokens, Word(x), "Expected name for #define statement");
            next!(tokens, Space);
            let val = next!(tokens, Number(x), "Expected value for #define statement");
            nodes.push(Directive::Define(name, val as u128).to_node());
        }
        "dyn" | "mem" => {
            next!(tokens, Space);

            let len = match expect_any!(tokens) {
                Token::Word(s) => match s.as_str() {
                    "byte" => 1,
                    "word" => 2,
                    t => err!("Invalid type for #{directive:?} definition: {t}")?,
                },
                Token::Number(l) => l,
                _ => err!("Expected length after '#{directive:?}'")?,
            };

            next!(tokens, Space);

            let name = next!(tokens, Word(x), "Expected name after '#{directive:?}'");

            if directive == "mem" {
                next!(tokens, Space);

                let mut val = vec![];
                if len == 1 {
                    let v = next!(tokens, Number(x), "Expected value after #mem assignment");
                    val.push(v as u8);
                } else if len == 2 {
                    let v = next!(tokens, Number(x), "Expected value after #mem assignment");
                    val.push(v as u8);
                    val.push((v >> 8) as u8);
                } else {
                    next!(
                        tokens,
                        BracketOpen,
                        "Expected '[0, 0, ...]' for #mem assignments longer than 2"
                    );
                    while let Some(next_num) = tokens.next() {
                        match next_num {
                            Token::Number(n) => {
                                val.push(n as u8);
                                while let Some(next_num) = tokens.peek() {
                                    match next_num {
                                        Token::Comma => break,
                                        Token::BracketClose => break,
                                        Token::Space | Token::NewLine => {
                                            tokens.next();
                                        }
                                        other => err!("Expected [0, 0, 0, ...], got: {other:?}")?,
                                    }
                                }
                            }
                            Token::Comma => continue,
                            Token::Space | Token::NewLine => continue,
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

                nodes.push(Directive::Rom(name, val).to_node())
            } else {
                nodes.push(Directive::Dynamic(name, len as u128).to_node())
            }
        }
        "macro" => {
            ignore_line_space!(tokens);
            let name = next!(tokens, Word(x), "Expected macro name");
            ignore_space!(tokens);
            next!(tokens, BracketOpen, "Expected macro args");
            ignore_space!(tokens);

            let mut args = vec![];
            while let Some(next) = tokens.next() {
                ignore_space!(tokens);
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

            next!(tokens, Colon, "Invalid macro syntax");

            let mut body = vec![];

            while let Some(next) = tokens.next() {
                match next {
                    Token::NewLine => {
                        if tokens.peek() == Some(&Token::NewLine) {
                            break;
                        }
                        body.push(Token::NewLine);
                    }
                    oth => body.push(oth),
                }
            }

            let mac_nodes = lex(body)?
                .into_iter()
                .map(|mn| match mn {
                    AstNode::Instruction(inst) => inst,
                    _ => panic!(
                        "Macro body for '{}' should only contain instructions",
                        &name
                    ),
                })
                .collect::<Vec<_>>();

            nodes.push(Macro::new(name, args, mac_nodes).to_node())
        }
        _ => err!("Invalid directive: '#{directive}'")?,
    };
    Ok(())
}
