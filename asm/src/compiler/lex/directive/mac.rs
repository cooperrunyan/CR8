use crate::compiler::lex::lexable::*;
use crate::compiler::lex::node::{Instruction, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct Macro<'m> {
    pub id: &'m str,
    pub captures: Vec<MacroCapture<'m>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MacroCapture<'m> {
    pub args: Vec<MacroCaptureArg<'m>>,
    pub content: Vec<Instruction<'m>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MacroCaptureArgType {
    Register,
    Imm8,
    Imm16,
    Imm8OrRegister,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MacroCaptureArg<'m> {
    pub id: &'m str,
    pub ty: MacroCaptureArgType,
}

impl<'b> Lexable<'b> for Macro<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace(buf);
        let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
        let mut mac = Macro {
            id,
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

impl<'b> Lexable<'b> for MacroCapture<'b> {
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
            Err(LexError::Expected(",".to_string()))?;
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
            raw = ignore_whitespace(raw);
            let (args, r) = Vec::<Value>::lex(raw)?;
            raw = r;
            content.push(Instruction { id, args });
        }

        Ok((MacroCapture { args, content }, buf))
    }
}

impl<'b> Lexable<'b> for MacroCaptureArg<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_' || c == '$')?;
        let buf = ignore_whitespace(buf);
        let buf = expect(buf, ":")?;
        let buf = ignore_whitespace(buf);
        let (ty, buf) = MacroCaptureArgType::lex(buf)?;

        Ok((MacroCaptureArg { id, ty }, buf))
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
            if (ty0 == "reg" && ty1 == "imm8") || (ty1 == "reg" && ty0 == "imm8") {
                return Ok((Self::Imm8OrRegister, buf));
            }
            return Err(LexError::Expected("`reg`, `imm8` or `imm16`".to_string()));
        }

        Ok((
            match ty0 {
                "reg" => Self::Register,
                "imm8" => Self::Imm8,
                "imm16" => Self::Imm16,
                _ => Err(LexError::UnknownSymbol(ty0.to_string()))?,
            },
            buf,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::lex::*;

    #[test]
    fn lex_macro<'s>() -> Result<(), Box<dyn std::error::Error>> {
        let (mac, remaining) = Macro::lex(
            r#"jnz: {
                ($addr: imm16, $if: imm8 | reg) => {
                    ldhl $addr
                    jnz $if
                }
                ($addr: imm8, $if: imm8 | reg) => {
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
    fn lex_macro_capture<'s>() -> Result<(), Box<dyn std::error::Error>> {
        let (cap, remaining) = MacroCapture::lex(
            r#"($addr: imm16, $if: imm8 | reg) => {
                ldhl $addr
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
                    id: "ldhl",
                    args: vec![Value::MacroVariable("$addr")]
                },
                Instruction {
                    id: "jnz",
                    args: vec![Value::MacroVariable("$if")]
                },
            ]
        );
        assert_eq!(
            cap.args,
            vec![
                MacroCaptureArg {
                    id: "$addr",
                    ty: MacroCaptureArgType::Imm16
                },
                MacroCaptureArg {
                    id: "$if",
                    ty: MacroCaptureArgType::Imm8OrRegister
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn lex_macro_capture_arg_imm16<'s>() -> Result<(), Box<dyn std::error::Error>> {
        let (arg, remaining) = MacroCaptureArg::lex(r#"$addr: imm16"#)?;

        assert!(remaining.is_empty());
        assert!(arg.id == "$addr");
        assert!(arg.ty == MacroCaptureArgType::Imm16);

        Ok(())
    }

    #[test]
    fn lex_macro_capture_arg_either<'s>() -> Result<(), Box<dyn std::error::Error>> {
        let (arg, remaining) = MacroCaptureArg::lex(r#"$addr: imm8 | reg"#)?;

        assert!(remaining.is_empty());
        assert!(arg.id == "$addr");
        assert!(arg.ty == MacroCaptureArgType::Imm8OrRegister);

        Ok(())
    }

    #[test]
    fn lex_macro_capture_arg_type<'s>() -> Result<(), Box<dyn std::error::Error>> {
        let (arg, remaining) = MacroCaptureArgType::lex(r#"imm16"#)?;
        assert!(arg == MacroCaptureArgType::Imm16);
        assert!(remaining.is_empty());

        let (arg, remaining) = MacroCaptureArgType::lex(r#"imm8"#)?;
        assert!(arg == MacroCaptureArgType::Imm8);
        assert!(remaining.is_empty());

        let (arg, remaining) = MacroCaptureArgType::lex(r#"reg"#)?;
        assert!(arg == MacroCaptureArgType::Register);
        assert!(remaining.is_empty());

        let (arg, remaining) = MacroCaptureArgType::lex(r#"reg | imm8"#)?;
        assert!(arg == MacroCaptureArgType::Imm8OrRegister);
        assert!(remaining.is_empty());

        let (arg, remaining) = MacroCaptureArgType::lex(r#"imm8 | reg"#)?;
        assert!(arg == MacroCaptureArgType::Imm8OrRegister);
        assert!(remaining.is_empty());

        Ok(())
    }
}
