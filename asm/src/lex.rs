use crate::{ast::AstNode, token::Token};

pub fn lex<'n>(tokens: Vec<Token>) -> Vec<AstNode<'n>> {
    let mut nodes = vec![];
    let mut tokens = tokens.into_iter().peekable();

    macro_rules! skip_spaces {
        () => {
            while tokens.peek() == Some(&Token::Space) {
                tokens.next();
            }
        };
    }

    macro_rules! skip_lines {
        () => {
            while tokens.peek() == Some(&Token::Space) || tokens.peek() == Some(&Token::NewLine) {
                tokens.next();
            }
        };
    }

    while let Some(token) = tokens.next() {
        match token {
            Token::Space | Token::NewLine => continue,
            Token::Directive => {
                let Some(Token::Word(directive)) = tokens.next() else {
                    panic!("Expected a word after '#'");
                };
                match directive.as_str() {
                    "include" => {
                        if Some(Token::Space) != tokens.next() {
                            panic!("Invalid syntax")
                        };
                        let Some(Token::String(path)) = tokens.next() else {
                            panic!("Expected path after '#include'")
                        };
                        nodes.push(AstNode::Import(path));
                    }
                    "origin" => {
                        if Some(Token::Space) != tokens.next() {
                            panic!("Invalid syntax")
                        };
                        let Some(Token::Number(addr)) = tokens.next() else {
                            panic!("Expected address for ram start after '#origin'")
                        };
                        nodes.push(AstNode::RamOriginDef(addr as u128));
                    }
                    "define" => {
                        if Some(Token::Space) != tokens.next() {
                            panic!("Invalid syntax")
                        };
                        let Some(Token::Word(name)) = tokens.next() else {
                            panic!("Expected name for #define statement")
                        };
                        skip_spaces!();
                        let Some(Token::Number(val)) = tokens.next() else {
                            panic!("Expected value for #define statement")
                        };
                        nodes.push(AstNode::StaticDef(name, val as u128));
                    }
                    "dyn" | "mem" => {
                        if Some(Token::Space) != tokens.next() {
                            panic!("Invalid syntax")
                        };

                        let len = match tokens.next() {
                            Some(Token::Word(ty)) => match ty.as_str() {
                                "byte" => 1,
                                "word" => 2,
                                t => panic!("Invalid type for #{directive:?} definition: {t}"),
                            },
                            Some(Token::Number(len)) => len,
                            Some(other) => {
                                panic!("Invalid type for #{directive:?} definition: {other:#?}")
                            }
                            None => panic!("Expected length after '#{directive:?}'"),
                        };

                        if Some(Token::Space) != tokens.next() {
                            panic!("Invalid syntax")
                        };

                        let name = match tokens.next() {
                            Some(Token::Word(n)) => n,
                            other => panic!("Expected name after '#{directive:?}'. Got: {other:?}"),
                        };

                        if directive == "mem" {
                            if Some(Token::Space) != tokens.next() {
                                panic!("Invalid syntax")
                            };
                            let mut val = vec![];
                            if len == 1 {
                                let Some(Token::Number(v)) = tokens.next() else {
                                    panic!("Expected value after #mem assignment")
                                };
                                val.push(v as u8);
                            } else if len == 2 {
                                let Some(Token::Number(v)) = tokens.next() else {
                                    panic!("Expected value after #mem assignment")
                                };
                                val.push(v as u8);
                                val.push((v >> 8) as u8);
                            } else {
                                let Some(Token::BracketOpen) = tokens.next() else {
                                    panic!(
                                        "Expected '[0, 0, ...]' for #mem assignments longer than 2"
                                    )
                                };
                                while let Some(next_num) = tokens.next() {
                                    match next_num {
                                        Token::Number(n) => val.push(n as u8),
                                        Token::Comma => continue,
                                        Token::Space | Token::NewLine => continue,
                                        Token::BracketClose => break,
                                        other => panic!("Expected [0, 0, 0, ...], got: {other:?}"),
                                    }
                                }
                            }

                            if val.len() != len as usize {
                                panic!(
                                    "Expected {name} to be {len} bytes long, got {} bytes",
                                    val.len()
                                )
                            }

                            nodes.push(AstNode::Rom(name, val))
                        } else {
                            nodes.push(AstNode::RamAlloc(name, len as u128))
                        }
                    }
                    "macro" => {
                        skip_lines!();
                        let Some(Token::Word(name)) = tokens.next() else {
                            panic!("Expected macro name")
                        };
                        skip_spaces!();
                        let Some(Token::BracketOpen) = tokens.next() else {
                            panic!("Expected macro args after name")
                        };
                        let mut args = vec![];
                        skip_spaces!();
                        while let Some(next) = tokens.next() {
                            skip_spaces!();
                            match next {
                                Token::Word(arg) => args.push(arg),
                                Token::Comma => continue,
                                Token::BracketClose => break,
                                oth => panic!("Unexpected value: {oth:?}"),
                            }
                        }
                        let Some(Token::Colon) = tokens.next() else {
                            panic!("Invalid macro syntax")
                        };
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

                        dbg!(name, args, body);

                        todo!("Create a macro from args name and body");
                    }
                    _ => panic!("Invalid directive: '#{directive}'"),
                }
            }
            _ => {
                // todo!("Lex non-directives")
            }
        }
    }

    nodes
}
