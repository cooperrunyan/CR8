use crate::compiler::lex::lexable::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Use {
    File(String),
    Module(String),
}

impl ToString for Use {
    fn to_string(&self) -> String {
        match self {
            Self::File(f) | Self::Module(f) => f.to_string(),
        }
    }
}

impl<'b> Lexable<'b> for Use {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        if let Ok(buf) = expect(buf, "\"") {
            let (file, buf) = collect_until(buf, |c| c == '"')?;
            let buf = expect(buf, "\"")?;
            return Ok((Self::File(file.to_string()), buf));
        }

        let (module, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == ':' || c == '_')?;
        Ok((Self::Module(module.to_string()), buf))
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::lex::*;

    #[test]
    fn lex_import_str() -> Result<(), Box<dyn std::error::Error>> {
        let (imp, remaining) = Use::lex(r#""./test.asm""#)?;

        assert!(remaining.is_empty());
        assert!(imp == Use::File("./test.asm".to_string()));

        Ok(())
    }

    #[test]
    fn lex_import_mod() -> Result<(), Box<dyn std::error::Error>> {
        let (imp, remaining) = Use::lex(r#"std::math"#)?;

        assert!(remaining.is_empty());
        assert!(imp == Use::Module("std::math".to_string()));

        Ok(())
    }
}
