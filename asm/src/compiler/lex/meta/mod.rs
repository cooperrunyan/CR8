use crate::compiler::lex::lexable::*;
use anyhow::bail;

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

impl<'b> Lexable<'b> for Meta {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = expect(buf, "#[")?;
        let buf = ignore_whitespace(buf);

        let (dir, buf) = collect_until(buf, |c| c == ']')?;
        let buf = expect(buf, "]")?;
        let (word, dir) = collect_while(dir, |c| c.is_alphabetic())?;
        let dir = ignore_whitespace(dir);

        match word {
            "main" => {
                expect_complete(dir)?;
                let buf = ignore_whitespace(buf);
                let b = buf;
                let (label, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
                let buf = ignore_whitespace(buf);
                let _ = expect(buf, ":")?;
                Ok((Meta::Main(label.to_string()), b))
            }
            "macro" => {
                expect_complete(dir)?;
                let buf = ignore_whitespace(buf);
                let (mac, buf) = Macro::lex(buf)?;
                Ok((Meta::Macro(mac), buf))
            }
            "static" => {
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, "(")?;
                let dir = ignore_whitespace(dir);
                let (id, dir) = collect_while(dir, |c| c.is_alphanumeric() || c == '_')?;
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, ":")?;
                let dir = ignore_whitespace(dir);
                let (num, dir) = usize::lex(dir)?;
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, ")")?;
                expect_complete(dir)?;
                Ok((Self::Static(id.to_string(), num), buf))
            }
            "use" => {
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, "(")?;
                let dir = ignore_whitespace(dir);
                let (import, dir) = Use::lex(dir)?;
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, ")")?;
                expect_complete(dir)?;
                Ok((Self::Use(import), buf))
            }
            "dyn" => {
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, "(")?;
                let dir = ignore_whitespace(dir);
                if let Ok(dir) = expect(dir, "&") {
                    let (org, dir) = usize::lex(dir)?;
                    let dir = expect(dir, ")")?;
                    expect_complete(dir)?;
                    return Ok((Self::DynOrigin(org), buf));
                }
                let (id, dir) = collect_while(dir, |c| c.is_alphanumeric() || c == '_')?;
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, ":")?;
                let dir = ignore_whitespace(dir);
                let (num, dir) = usize::lex(dir)?;
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, ")")?;
                expect_complete(dir)?;
                Ok((Self::Dyn(id.to_string(), num), buf))
            }
            "const" => {
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, "(")?;
                let dir = ignore_whitespace(dir);
                let (id, dir) = collect_while(dir, |c| c.is_alphanumeric() || c == '_')?;
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, ")")?;
                expect_complete(dir)?;
                let buf = ignore_whitespace(buf);
                let (explicit, buf) = Constant::lex(buf)?;
                Ok((Self::Constant(id.to_string(), explicit), buf))
            }
            oth => bail!("Unknown directive {oth:#?} at {buf}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stat() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = Meta::lex("#[static(HELLO: 0xFF00)]")?;
        assert_eq!(dir, Meta::Static("HELLO".to_string(), 0xFF00));

        let (dir, _) = Meta::lex("#[static(HELLO: 2)]")?;
        assert_eq!(dir, Meta::Static("HELLO".to_string(), 2));

        let (dir, _) = Meta::lex("#[static(HELLO: 0b1001)]")?;
        assert_eq!(dir, Meta::Static("HELLO".to_string(), 0b1001));

        Ok(())
    }

    #[test]
    fn lex_dyn() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = Meta::lex("#[dyn(TEST: 4)]")?;
        assert_eq!(dir, Meta::Dyn("TEST".to_string(), 4));

        let (dir, _) = Meta::lex("#[dyn(&0xC000)]")?;
        assert_eq!(dir, Meta::DynOrigin(0xC000));

        Ok(())
    }
}
