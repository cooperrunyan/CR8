use crate::lex_enum;

use super::{expect, ignore_whitespace, ignore_whitespace_noline, Lexable};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Pragma {
    #[default]
    None,
    Micro,
}

impl<'b> Lexable<'b> for Pragma {
    fn lex(buf: &'b str) -> super::LexResult<'b, Self> {
        let buf = ignore_whitespace(buf);
        let buf = match expect(buf, "#![") {
            Ok(buf) => buf,
            Err(_) => return Ok((Self::None, buf)),
        };
        let buf = ignore_whitespace_noline(buf);
        let (variant, buf) = lex_enum! { buf;
            "micro" => Pragma::Micro,
        }?;
        let buf = ignore_whitespace_noline(buf);
        let buf = expect(buf, "]")?;
        Ok((variant, buf))
    }
}
