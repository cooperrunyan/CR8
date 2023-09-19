use anyhow::{anyhow, bail, Result};

use crate::compiler::ast::{AstNode, Capture, Instruction, Macro, MacroArg, ToNode};
use crate::compiler::tokenizer::Token;

use super::Lexer;

impl Lexer {
    pub fn lex_macro(&mut self) -> Result<()> {
        ignore!(self, Token::NewLine | Token::Space);
        let name = expect!(self, "Expected macro name", Word(x));
        ignore!(self, Token::Space);
        expect!(self, "Invalid syntax", match is_colon);
        ignore!(self, Token::Space);

        expect!(self, "Expected {{}} after '#macro _'", match is_mustache_open);

        let mut captures = vec![];

        while_next!(self, next, {
            match &next.token {
                Token::MustacheClose => break,
                Token::ParenOpen => {
                    let capture = self.lex_macro_capture()?;
                    captures.push(capture);
                }
                Token::Space | Token::NewLine => continue,
                oth => bail!("Unexpected {oth:?} in macro capture body"),
            }
        });

        self.nodes.push(Macro::new(name, captures).to_node());

        Ok(())
    }

    fn lex_macro_capture(&mut self) -> Result<Capture> {
        let args = self.lex_macro_capture_args()?;

        ignore!(self, Token::Space);
        expect!(self, "Expected '=>'", match is_right_arrow);
        ignore!(self, Token::Space);
        expect!(self, "Expected '{{'", match is_mustache_open);

        let body = self.lex_macro_body()?;

        Ok(Capture { args, body })
    }

    fn lex_macro_capture_args(&mut self) -> Result<Vec<MacroArg>> {
        let mut args: Vec<MacroArg> = vec![];
        let mut end = false;
        while_next!(self, next, {
            if end {
                break;
            }
            match &next.token {
                Token::ParenClose => break,
                Token::Dollar => {
                    let name = expect!(self, "Expected arg name after '$'", Word(x));
                    ignore!(self, Token::Space);
                    expect!(self, "Expected ':' to declare type", match is_colon);
                    let mut types = String::new();
                    let mut grab_type = true;

                    while_next!(self, next, {
                        match &next.token {
                            Token::Comma => break,
                            Token::ParenClose => {
                                end = true;
                                break;
                            }
                            Token::Space | Token::NewLine => continue,
                            Token::Word(t) => {
                                if grab_type {
                                    grab_type = false;
                                    types.push_str(t);
                                } else {
                                    bail!("Unexpected word {:?} in macro capture args", next.token)
                                }
                            }
                            Token::Pipe => {
                                if grab_type {
                                    bail!("Unexpected | in macro capture args");
                                }
                                types.push('|');
                                grab_type = true;
                            }
                            oth => bail!("Unexpected {oth:?} in macro capture args"),
                        }
                    });

                    let arg = match types.as_str() {
                        "reg|imm8" | "imm8|reg" => MacroArg::ImmReg(name),
                        "reg" => MacroArg::Register(name),
                        "imm8" => MacroArg::Imm8(name),
                        "imm16" => MacroArg::Imm16(name),
                        oth => bail!("Unknown macro type: {oth:#?}"),
                    };
                    args.push(arg);

                    if end {
                        break;
                    }
                }
                Token::Space => {}
                oth => bail!("Unexpected {oth:?} in macro capture args"),
            }
        });

        Ok(args)
    }

    fn lex_macro_body(&mut self) -> Result<Vec<Instruction>> {
        let mut tokens = vec![];

        while_next!(self, next, {
            match &next.token {
                Token::MustacheClose => break,
                _ => tokens.push(next),
            }
        });

        let nodes = Lexer::new(tokens, self.file.clone()).lex()?.nodes;

        Ok(nodes
            .into_iter()
            .map(|n| match n {
                AstNode::Instruction(i) => i,
                _ => panic!("Expected macro body to only have instructions"),
            })
            .collect())
    }
}
