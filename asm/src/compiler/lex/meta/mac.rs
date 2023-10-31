use crate::compiler::lex::lexable::*;
use crate::compiler::lex::node::{Instruction, Value};
use crate::{lex_enum, repeated, token};

use anyhow::bail;

#[derive(Debug, PartialEq, Eq)]
pub struct Macro {
    pub id: String,
    pub captures: Vec<MacroCapture>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MacroCapture {
    pub args: Vec<MacroCaptureArg>,
    pub content: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MacroCaptureArgType {
    Register,
    Literal,
    Expr,
    Any,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MacroCaptureArg {
    pub id: String,
    pub ty: MacroCaptureArgType,
}

impl<'b> Lexable<'b> for Macro {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace(buf);
        let (id, buf) = token!(buf; '_')?;

        let buf = ignore_whitespace(buf);
        let buf = expect(buf, ":")?;
        let buf = ignore_whitespace(buf);

        let (captures, buf) = repeated!("{" buf "}" {
            MacroCapture::lex(buf)?
        });

        Ok((
            Macro {
                id: id.to_string(),
                captures,
            },
            buf,
        ))
    }
}

impl<'b> Lexable<'b> for MacroCapture {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let (args, buf) = repeated!("(" buf "," ")" {
            MacroCaptureArg::lex(buf)?
        });

        let buf = ignore_whitespace(buf);
        let buf = expect(buf, "=>")?;
        let buf = ignore_whitespace(buf);

        let (content, buf) = repeated!("{" buf "}" {
            let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
            let buf = ignore_whitespace_noline(buf);
            if let Ok(buf) = expect(buf, "\n") {
                (Instruction {
                    id: id.to_string(),
                    args: vec![],
                }, buf)
            } else {
                let (args, buf) = Vec::<Value>::lex(buf)?;
                (Instruction {
                    id: id.to_string(),
                    args,
                }, buf)
            }
        });

        Ok((MacroCapture { args, content }, buf))
    }
}

impl<'b> Lexable<'b> for MacroCaptureArg {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let (id, buf) = token!(buf; '_' | '$')?;
        let buf = ignore_whitespace(buf);
        let buf = expect(buf, ":")?;
        let buf = ignore_whitespace(buf);
        let (ty, buf) = MacroCaptureArgType::lex(buf)?;

        Ok((
            MacroCaptureArg {
                id: id.to_string(),
                ty,
            },
            buf,
        ))
    }
}

impl<'b> Lexable<'b> for MacroCaptureArgType {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace(buf);

        lex_enum!(buf;
            "reg" => Self::Register,
            "lit" => Self::Literal,
            "expr" => Self::Expr,
            "any" => Self::Any,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::lex::*;

    #[test]
    fn lex_macro() -> Result<(), Box<dyn std::error::Error>> {
        let (mac, remaining) = Macro::lex(
            r#"jnz: {
                ($addr: expr, $if: any) => {
                    ldxy $addr
                    jnz $if
                }
                ($addr: lit, $if: any) => {
                    jnz $if
                }
            }"#,
        )?;

        assert!(remaining.is_empty());
        assert!(mac.id == "jnz");
        assert!(mac.captures.len() == 2);

        Ok(())
    }

    #[test]
    fn lex_macro_capture() -> Result<(), Box<dyn std::error::Error>> {
        let (cap, remaining) = MacroCapture::lex(
            r#"($addr: expr, $if: any) => {
                ldxy $addr
                jnz $if
            }"#,
        )?;

        assert!(remaining.is_empty());
        assert!(cap.args.len() == 2);
        assert!(cap.content.len() == 2);
        assert_eq!(
            cap.content,
            vec![
                Instruction {
                    id: "ldxy".to_string(),
                    args: vec![Value::MacroVariable("$addr".to_string())]
                },
                Instruction {
                    id: "jnz".to_string(),
                    args: vec![Value::MacroVariable("$if".to_string())]
                },
            ]
        );
        assert_eq!(
            cap.args,
            vec![
                MacroCaptureArg {
                    id: "$addr".to_string(),
                    ty: MacroCaptureArgType::Expr
                },
                MacroCaptureArg {
                    id: "$if".to_string(),
                    ty: MacroCaptureArgType::Any
                },
            ]
        );

        Ok(())
    }
}
