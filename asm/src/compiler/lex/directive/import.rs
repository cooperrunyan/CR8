use crate::compiler::lex::lexable::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Import<'d> {
    File(&'d str),
    Module(&'d str),
}

impl<'b> Lexable<'b> for Import<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        if let Ok(buf) = expect(buf, "\"") {
            let (file, buf) = collect_until(buf, |c| c == '"')?;
            let buf = expect(buf, "\"")?;
            return Ok((Self::File(file), buf));
        }

        let (module, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == ':')?;
        Ok((Self::Module(module), buf))
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::lex::*;

    #[test]
    fn lex_import_str<'s>() -> Result<(), Box<dyn std::error::Error>> {
        let (imp, remaining) = Import::lex(r#""./test.asm""#)?;

        assert!(remaining.is_empty());
        assert!(imp == Import::File("./test.asm"));

        Ok(())
    }

    #[test]
    fn lex_import_mod<'s>() -> Result<(), Box<dyn std::error::Error>> {
        let (imp, remaining) = Import::lex(r#"std::math"#)?;

        assert!(remaining.is_empty());
        assert!(imp == Import::Module("std::math"));

        Ok(())
    }
}
