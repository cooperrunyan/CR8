use std::iter::Peekable;
use std::path::PathBuf;
use std::vec::IntoIter;

use crate::ast::{AstNode, Directive, Macro, MacroArg, ToNode};
use crate::err;
use crate::lex::lex;
use crate::token::Token;

use super::LexError;

pub fn lex_directive(
    file: &PathBuf,
    line: &mut u128,
    tokens: &mut Peekable<IntoIter<Token>>,
    nodes: &mut Vec<AstNode>,
) -> Result<(), LexError> {
    macro_rules! defnext {
        ($name:ident, $t:ident$(($a:ident))?) => {
            macro_rules! $name {
                ($err:expr) => {{
                    let Some(w) = next!(tokens, $t $(($a))?) else {
                        err!(line,  file, $err)?
                    };
                    w
                }};
            }
        };
    }

    defnext!(word, Word(x));
    defnext!(num, Number(x));
    defnext!(stri, String(x));
    defnext!(brackopen, BracketOpen);
    defnext!(space, Space);
    defnext!(colon, Colon);

    let directive = word!("Expected directive after: '#'");
    match directive.as_str() {
        "include" => {
            space!("Syntax error");
            let path = stri!("Expected path after #include statement");
            nodes.push(Directive::Import(path).to_node());
        }
        "origin" => {
            space!("Syntax error");
            let addr = num!("Expected address after #origin");
            nodes.push(Directive::Origin(addr as u128).to_node());
        }
        "define" => {
            space!("Syntax error");
            let name = word!("Expected name for #define statement");
            space!("Syntax error");
            let val = num!("Expected value for #define statement");
            nodes.push(Directive::Define(name, val as u128).to_node());
        }
        "dyn" | "mem" => {
            space!("Syntax error");

            let len = match tokens.next() {
                Some(Token::Word(s)) => match s.as_str() {
                    "byte" => 1,
                    "word" => 2,
                    t => err!(
                        line,
                        file,
                        "Invalid type for #{directive:?} definition: {t}"
                    )?,
                },
                Some(Token::Number(l)) => l,
                _ => err!(line, file, "Expected length after '#{directive:?}'")?,
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
                    while let Some(next_num) = tokens.next() {
                        match next_num {
                            Token::Number(n) => {
                                val.push(n as u8);
                                while let Some(next_num) = tokens.peek() {
                                    match next_num {
                                        Token::Comma => break,
                                        Token::BracketClose => break,
                                        Token::Space => {
                                            tokens.next();
                                        }
                                        Token::NewLine => {
                                            *line += 1;
                                            tokens.next();
                                        }
                                        other => err!(
                                            line,
                                            file,
                                            "Expected [0, 0, 0, ...], got: {other:?}"
                                        )?,
                                    }
                                }
                            }
                            Token::Comma => continue,
                            Token::Space => continue,
                            Token::NewLine => {
                                *line += 1;
                                continue;
                            }
                            Token::BracketClose => break,
                            other => err!(line, file, "Expected [0, 0, 0, ...], got: {other:?}")?,
                        }
                    }
                }

                if val.len() != len as usize {
                    err!(
                        line,
                        file,
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
            while tokens.peek() == Some(&Token::Space) || tokens.peek() == Some(&Token::NewLine) {
                if tokens.peek() == Some(&Token::NewLine) {
                    *line += 1;
                }
                tokens.next();
            }
            let name = word!("Expected macro name");
            ignore_space!(tokens);
            brackopen!("Expected macro args");
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
                            err!(line,file,"Macro arg should start with 'i' 'ir' 'r' or 'a' to signify its type")?
                        }
                    }
                    Token::Comma => continue,
                    Token::BracketClose => break,
                    oth => err!(line, file, "Unexpected value: {oth:?}")?,
                }
            }

            colon!("Invalid macro syntax");

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

            let mac_nodes = lex(body, file)?
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
        _ => err!(line, file, "Invalid directive: '#{directive}'")?,
    };
    Ok(())
}
