use crate::compiler::lex::lexable::*;

mod bytes;
mod import;
mod mac;

pub use bytes::*;
pub use import::*;
pub use mac::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Directive<'d> {
    Boot(&'d str),
    ExplicitBytes(&'d str, ExplicitBytes),
    Dyn(&'d str, usize),
    DynOrigin(usize),
    Macro(Macro<'d>),
    Static(&'d str, usize),
    Use(Import<'d>),
}

impl<'b> Lexable<'b> for Directive<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = expect(buf, "#[")?;
        let buf = ignore_whitespace(buf);

        let (dir, buf) = collect_until(buf, |c| c == ']')?;
        let buf = expect(buf, "]")?;
        let (word, dir) = collect_while(dir, |c| c.is_alphabetic())?;
        let dir = ignore_whitespace(dir);

        match word {
            "boot" => {
                expect_complete(dir)?;
                let buf = ignore_whitespace(buf);
                let b = buf;
                let (label, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
                let buf = ignore_whitespace(buf);
                let _ = expect(buf, ":")?;
                Ok((Directive::Boot(label), b))
            }
            "macro" => {
                expect_complete(dir)?;
                let buf = ignore_whitespace(buf);
                let (mac, buf) = Macro::lex(buf)?;
                Ok((Directive::Macro(mac), buf))
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
                Ok((Self::Static(id, num), buf))
            }
            "use" => {
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, "(")?;
                let dir = ignore_whitespace(dir);
                let (import, dir) = Import::lex(dir)?;
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
                Ok((Self::Dyn(id, num), buf))
            }
            "explicit" => {
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, "(")?;
                let dir = ignore_whitespace(dir);
                let (id, dir) = collect_while(dir, |c| c.is_alphanumeric() || c == '_')?;
                let dir = ignore_whitespace(dir);
                let dir = expect(dir, ")")?;
                expect_complete(dir)?;
                let buf = ignore_whitespace(buf);
                let (explicit, buf) = ExplicitBytes::lex(buf)?;
                Ok((Self::ExplicitBytes(id, explicit), buf))
            }
            oth => Err(LexError::UnknownSymbol(oth.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stat() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = Directive::lex("#[static(HELLO: 0xFF00)]")?;
        assert_eq!(dir, Directive::Static("HELLO", 0xFF00));

        let (dir, _) = Directive::lex("#[static(HELLO: 2)]")?;
        assert_eq!(dir, Directive::Static("HELLO", 2));

        let (dir, _) = Directive::lex("#[static(HELLO: 0b1001)]")?;
        assert_eq!(dir, Directive::Static("HELLO", 0b1001));

        Ok(())
    }

    #[test]
    fn lex_dyn() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = Directive::lex("#[dyn(TEST: 4)]")?;
        assert_eq!(dir, Directive::Dyn("TEST", 4));

        let (dir, _) = Directive::lex("#[dyn(&0xC000)]")?;
        assert_eq!(dir, Directive::DynOrigin(0xC000));

        Ok(())
    }
}
