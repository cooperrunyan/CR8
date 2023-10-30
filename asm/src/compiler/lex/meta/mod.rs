use crate::compiler::lex::lexable::*;
use crate::lex_enum;

mod bytes;
mod import;
mod mac;

pub use bytes::*;
pub use import::*;
pub use mac::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Meta {
    Main(String),
    Constant(String, Constant),
    Dyn(String, usize),
    DynOrigin(usize),
    Macro(Macro),
    Static(String, usize),
    Use(Use),
}

#[derive(Debug, Clone, Copy)]
pub enum MetaKind {
    Main,
    Constant,
    Dyn,
    Macro,
    Static,
    Use,
}

impl<'b> Lexable<'b> for Meta {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = expect(buf, "#[")?;
        let buf = ignore_whitespace_noline(buf);

        let (word, buf) = lex_enum! { buf;
            "main" => MetaKind::Main,
            "macro" => MetaKind::Macro,
            "static" => MetaKind::Static,
            "const" => MetaKind::Constant,
            "use" => MetaKind::Use,
            "dyn" => MetaKind::Dyn,
        }
        .map_err(|e| e.context("Unknown meta keyword"))?;

        match word {
            MetaKind::Main => {
                let buf = expect(buf, "]")?;
                let buf = ignore_whitespace(buf);
                let b = buf;
                let (label, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
                let buf = ignore_whitespace(buf);
                let _ = expect(buf, ":")?;
                Ok((Meta::Main(label.to_string()), b))
            }
            MetaKind::Macro => {
                let buf = expect(buf, "]")?;
                let buf = ignore_whitespace(buf);
                let (mac, buf) = Macro::lex(buf)?;
                Ok((Meta::Macro(mac), buf))
            }
            MetaKind::Static => {
                let buf = ignore_whitespace(buf);
                let buf = expect(buf, "(")?;
                let buf = ignore_whitespace(buf);
                let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
                let buf = ignore_whitespace(buf);
                let buf = expect(buf, ":")?;
                let buf = ignore_whitespace(buf);
                let (num, buf) = usize::lex(buf)?;
                let buf = ignore_whitespace(buf);
                let buf = expect(buf, ")")?;
                let buf = expect(buf, "]")?;
                Ok((Self::Static(id.to_string(), num), buf))
            }
            MetaKind::Use => {
                let buf = ignore_whitespace(buf);
                let buf = expect(buf, "(")?;
                let buf = ignore_whitespace(buf);
                let (import, buf) = Use::lex(buf)?;
                let buf = ignore_whitespace(buf);
                let buf = expect(buf, ")")?;
                let buf = expect(buf, "]")?;
                Ok((Self::Use(import), buf))
            }
            MetaKind::Dyn => {
                let buf = ignore_whitespace(buf);
                let buf = expect(buf, "(")?;
                let buf = ignore_whitespace(buf);
                if let Ok(buf) = expect(buf, "&") {
                    let (org, buf) = usize::lex(buf)?;
                    let buf = expect(buf, ")")?;
                    let buf = expect(buf, "]")?;
                    return Ok((Self::DynOrigin(org), buf));
                }
                let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
                let buf = ignore_whitespace(buf);
                let buf = expect(buf, ":")?;
                let buf = ignore_whitespace(buf);
                let (num, buf) = usize::lex(buf)?;
                let buf = ignore_whitespace(buf);
                let buf = expect(buf, ")")?;
                let buf = expect(buf, "]")?;
                Ok((Self::Dyn(id.to_string(), num), buf))
            }
            MetaKind::Constant => {
                let buf = ignore_whitespace(buf);
                let buf = expect(buf, "(")?;
                let buf = ignore_whitespace(buf);
                let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
                let buf = ignore_whitespace(buf);
                let buf = expect(buf, ")")?;
                let buf = expect(buf, "]")?;
                let buf = ignore_whitespace(buf);
                let (explicit, buf) = Constant::lex(buf)?;
                Ok((Self::Constant(id.to_string(), explicit), buf))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stat() -> Result<(), Box<dyn std::error::Error>> {
        let (buf, _) = Meta::lex("#[static(HELLO: 0xFF00)]")?;
        assert_eq!(buf, Meta::Static("HELLO".to_string(), 0xFF00));

        let (buf, _) = Meta::lex("#[static(HELLO: 2)]")?;
        assert_eq!(buf, Meta::Static("HELLO".to_string(), 2));

        let (buf, _) = Meta::lex("#[static(HELLO: 0b1001)]")?;
        assert_eq!(buf, Meta::Static("HELLO".to_string(), 0b1001));

        Ok(())
    }

    #[test]
    fn lex_dyn() -> Result<(), Box<dyn std::error::Error>> {
        let (buf, _) = Meta::lex("#[dyn(TEST: 4)]")?;
        assert_eq!(buf, Meta::Dyn("TEST".to_string(), 4));

        let (buf, _) = Meta::lex("#[dyn(&0xC000)]")?;
        assert_eq!(buf, Meta::DynOrigin(0xC000));

        Ok(())
    }
}
