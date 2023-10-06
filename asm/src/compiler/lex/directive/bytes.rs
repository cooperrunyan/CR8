use crate::compiler::lex::lexable::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ExplicitBytes(pub Vec<u8>);

impl<'b> Lexable<'b> for ExplicitBytes {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let mut buf = expect(buf, "{")?;
        let mut bytes = vec![];
        loop {
            buf = ignore_whitespace(buf);
            if let Ok(buf) = expect(buf, "}") {
                return Ok((Self(bytes), buf));
            }
            let (byte, b) = usize::lex(buf)?;
            buf = b;

            bytes.push(byte as u8);
            buf = ignore_whitespace(buf);
            if let Ok(b) = expect(buf, ",") {
                buf = b;
                continue;
            }
            if let Ok(buf) = expect(buf, "}") {
                return Ok((Self(bytes), buf));
            }
            Err(LexError::Expected(",".to_string()))?;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::lex::*;

    #[test]
    fn lex_bytes<'s>() -> Result<(), Box<dyn std::error::Error>> {
        let (b, remaining) = ExplicitBytes::lex(r#"{ 0, 0, 1, 0 }"#)?;

        assert!(remaining.is_empty());
        assert!(b.0 == vec![0, 0, 1, 0]);

        Ok(())
    }
}
