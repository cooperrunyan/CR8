use crate::compiler::lex::lexable::*;
use crate::compiler::lex::node::{Instruction, Value};

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

#[derive(Debug, PartialEq, Eq)]
pub enum MacroCaptureArgType {
    Register,
    Literal,
    Expr,
    LiteralOrRegister,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MacroCaptureArg {
    pub id: String,
    pub ty: MacroCaptureArgType,
}

impl<'b> Lexable<'b> for Macro {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace(buf);
        let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
        let mut mac = Macro {
            id: id.to_string(),
            captures: vec![],
        };

        let buf = ignore_whitespace(buf);
        let buf = expect(buf, ":")?;
        let buf = ignore_whitespace(buf);
        let buf = expect(buf, "{")?;
        let mut buf = buf;

        loop {
            buf = ignore_whitespace(buf);
            if let Ok(buf) = expect(buf, "}") {
                return Ok((mac, buf));
            }

            let (cap, b) = MacroCapture::lex(buf)?;
            buf = b;
            mac.captures.push(cap);
        }
    }
}

impl<'b> Lexable<'b> for MacroCapture {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let mut buf = expect(buf, "(")?;
        let mut args = vec![];

        loop {
            buf = ignore_whitespace(buf);
            if let Ok(b) = expect(buf, ")") {
                buf = b;
                break;
            }

            let (arg, b) = MacroCaptureArg::lex(buf)?;
            buf = b;

            args.push(arg);
            buf = ignore_whitespace(buf);
            if let Ok(b) = expect(buf, ",") {
                buf = b;
                continue;
            }
            if let Ok(b) = expect(buf, ")") {
                buf = b;
                break;
            }
            bail!("Expected ',' or ')' after macro arg. \n\n{buf}");
        }

        let buf = ignore_whitespace(buf);
        let buf = expect(buf, "=>")?;
        let buf = ignore_whitespace(buf);
        let buf = expect(buf, "{")?;
        let buf = ignore_whitespace(buf);
        let (mut raw, buf) = collect_until(buf, |c| c == '}')?;
        let buf = expect(buf, "}")?;

        let mut content = vec![];

        loop {
            raw = ignore_whitespace(raw);
            if raw.is_empty() {
                break;
            }
            let (id, r) = collect_while(raw, |c| c.is_alphanumeric() || c == '_')?;
            raw = r;
            raw = ignore_whitespace_noline(raw);
            if let Ok(r) = expect(raw, "\n") {
                raw = r;
                content.push(Instruction {
                    id: id.to_string(),
                    args: vec![],
                });
            } else {
                let (args, r) = Vec::<Value>::lex(raw)?;
                raw = r;
                content.push(Instruction {
                    id: id.to_string(),
                    args,
                });
            }
        }

        Ok((MacroCapture { args, content }, buf))
    }
}

impl<'b> Lexable<'b> for MacroCaptureArg {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_' || c == '$')?;
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
        let (ty0, buf) = collect_while(buf, |c| c.is_alphanumeric())?;
        let buf = ignore_whitespace(buf);
        if let Ok(buf) = expect(buf, "|") {
            let buf = ignore_whitespace(buf);
            let (ty1, buf) = collect_while(buf, |c| c.is_alphanumeric())?;
            if (ty0 == "reg" && ty1 == "lit") || (ty1 == "reg" && ty0 == "lit") {
                return Ok((Self::LiteralOrRegister, buf));
            }
            bail!("Expected `reg`, `lit` or `expr` at {buf:#?}");
        }

        Ok((
            match ty0 {
                "reg" => Self::Register,
                "lit" => Self::Literal,
                "expr" => Self::Expr,
                _ => bail!("Unknown macro type {ty0:#?}"),
            },
            buf,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::lex::*;

    #[test]
    fn lex_macro() -> Result<(), Box<dyn std::error::Error>> {
        let (mac, remaining) = Macro::lex(
            r#"jnz: {
                ($addr: expr, $if: lit | reg) => {
                    ldxy $addr
                    jnz $if
                }
                ($addr: lit, $if: lit | reg) => {
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
            r#"($addr: expr, $if: lit | reg) => {
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
                    ty: MacroCaptureArgType::LiteralOrRegister
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn lex_macro_capture_arg_imm16() -> Result<(), Box<dyn std::error::Error>> {
        let (arg, remaining) = MacroCaptureArg::lex(r#"$addr: expr"#)?;

        assert!(remaining.is_empty());
        assert!(arg.id == "$addr");
        assert!(arg.ty == MacroCaptureArgType::Expr);

        Ok(())
    }

    #[test]
    fn lex_macro_capture_arg_either() -> Result<(), Box<dyn std::error::Error>> {
        let (arg, remaining) = MacroCaptureArg::lex(r#"$addr: lit | reg"#)?;

        assert!(remaining.is_empty());
        assert!(arg.id == "$addr");
        assert!(arg.ty == MacroCaptureArgType::LiteralOrRegister);

        Ok(())
    }

    #[test]
    fn lex_macro_capture_arg_type() -> Result<(), Box<dyn std::error::Error>> {
        let (arg, remaining) = MacroCaptureArgType::lex(r#"expr"#)?;
        assert!(arg == MacroCaptureArgType::Expr);
        assert!(remaining.is_empty());

        let (arg, remaining) = MacroCaptureArgType::lex(r#"lit"#)?;
        assert!(arg == MacroCaptureArgType::Literal);
        assert!(remaining.is_empty());

        let (arg, remaining) = MacroCaptureArgType::lex(r#"reg"#)?;
        assert!(arg == MacroCaptureArgType::Register);
        assert!(remaining.is_empty());

        let (arg, remaining) = MacroCaptureArgType::lex(r#"reg | lit"#)?;
        assert!(arg == MacroCaptureArgType::LiteralOrRegister);
        assert!(remaining.is_empty());

        let (arg, remaining) = MacroCaptureArgType::lex(r#"lit | reg"#)?;
        assert!(arg == MacroCaptureArgType::LiteralOrRegister);
        assert!(remaining.is_empty());

        Ok(())
    }
}
