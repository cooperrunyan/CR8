use crate::{lex_enum, surround_inline};

use super::{expect, ignore_whitespace, Lexable};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Pragma {
    #[default]
    None,
    Micro,
}

impl<'b> Lexable<'b> for Pragma {
    fn lex(buf: &'b str) -> super::LexResult<'b, Self> {
        let buf = ignore_whitespace(buf);

        match expect(buf, "#![") {
            Ok(_) => {}
            Err(_) => return Ok((Self::None, buf)),
        };

        let (variant, buf) = surround_inline!("#![" buf "]" {
            lex_enum! { buf;
                "micro" => Pragma::Micro,
            }?
        });

        Ok((variant, buf))
    }
}
