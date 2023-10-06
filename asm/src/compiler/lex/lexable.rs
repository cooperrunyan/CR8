use std::error::Error;
use std::fmt::Debug;
use std::num::ParseIntError;
use thiserror as e;

use crate::op::Operation;
use crate::reg::Register;

#[derive(e::Error)]
pub enum LexError {
    #[error("unexpected: {}", .0)]
    Unexpected(String),

    #[error("unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("expected: {}", .0)]
    Expected(String),

    #[error("cannot redefine: {}", .0)]
    Redefinition(String),

    #[error("unknown symbol: {}", .0)]
    UnknownSymbol(String),

    #[error("unknown register: {}", .0)]
    UnknownRegister(String),

    #[error("unknown operator at: {}", .0)]
    UnknownOperator(String),

    #[error("invalid number")]
    ParseNumberError(#[from] ParseIntError),
}

impl Debug for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}

pub type LexResult<'b, T> = Result<(T, &'b str), LexError>;

pub trait Lexable<'b>: Sized {
    fn lex(buf: &'b str) -> LexResult<'b, Self>;
}

pub fn ignore_comment<'b>(buf: &'b str) -> &'b str {
    if buf.starts_with(";") {
        if let Some(nl) = buf.find('\n') {
            ignore_comment(&buf[nl..].trim_start_matches(char::is_whitespace))
        } else {
            &""
        }
    } else {
        buf
    }
}

pub fn ignore_whitespace<'b>(buf: &'b str) -> &'b str {
    let buf = buf.trim_start_matches(char::is_whitespace);
    let buf = ignore_comment(buf);
    let buf = buf.trim_start_matches(char::is_whitespace);
    buf
}

pub fn ignore_whitespace_noline<'b>(buf: &'b str) -> &'b str {
    let buf = buf.trim_start_matches(&[' ', '\t']);
    let buf = ignore_comment(buf);
    let buf = buf.trim_start_matches(&[' ', '\t']);
    buf
}

pub fn collect<'b, M: Fn(char) -> bool>(buf: &'b str, check: M) -> LexResult<'b, &'b str> {
    for (i, ch) in buf.chars().enumerate() {
        if check(ch) {
            let remaining = &buf[i..];
            if i == 0 {
                return Err(LexError::Unexpected(buf.to_string()));
            }
            return if remaining.len() == 0 {
                Err(LexError::UnexpectedEndOfInput)
            } else {
                Ok(buf.split_at(i))
            };
        }
    }

    Ok((buf, ""))
}

pub fn collect_until<'b, M: Fn(char) -> bool>(buf: &'b str, check: M) -> LexResult<'b, &'b str> {
    collect(buf, check)
}

pub fn collect_while<'b, M: Fn(char) -> bool>(buf: &'b str, check: M) -> LexResult<'b, &'b str> {
    collect(buf, |ch| !check(ch))
}

pub fn expect_complete<'b>(buf: &'b str) -> LexResult<'b, ()> {
    let buf = ignore_whitespace(buf);
    if buf.len() != 0 {
        return Err(LexError::Unexpected(buf.to_string()));
    }
    Ok(((), buf))
}

pub fn expect<'b>(buf: &'b str, expect: &'static str) -> Result<&'b str, LexError> {
    if buf.starts_with(expect) {
        Ok(&buf[expect.len()..])
    } else {
        Err(LexError::Expected(expect.to_string()))
    }
}

impl<'b, T: Lexable<'b>> Lexable<'b> for Vec<T> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let mut values = vec![];
        let mut buf = buf;
        let buf = loop {
            buf = buf.trim_start_matches(&[' ', '\t']);
            if let Ok(b) = expect(buf, "\n") {
                buf = b;
                break buf;
            }
            let (val, b) = T::lex(buf)?;
            buf = b;
            buf = buf.trim_start_matches(&[' ', '\t']);
            values.push(val);
            if let Ok(b) = expect(buf, ",") {
                buf = b;
                continue;
            }

            break buf;
        };
        return Ok((values, buf));
    }
}

impl<'b> Lexable<'b> for usize {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let (radix, buf) = if let Ok(buf) = expect(buf, "0x") {
            (16, buf)
        } else if let Ok(buf) = expect(buf, "0b") {
            (2, buf)
        } else {
            (10, buf)
        };

        let (num, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
        let num = &num.replace('_', "");
        match usize::from_str_radix(&num, radix) {
            Ok(val) => Ok((val, buf)),
            Err(e) => Err(LexError::ParseNumberError(e)),
        }
    }
}

impl<'b> Lexable<'b> for Register {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        use Register as R;
        let buf = expect(buf, "%")?;
        let (reg, buf) = collect_while(buf, |c| c.is_alphabetic())?;
        let reg = match reg {
            "a" => R::A,
            "b" => R::B,
            "c" => R::C,
            "d" => R::D,
            "z" => R::Z,
            "l" => R::L,
            "h" => R::H,
            "f" => R::F,
            _ => Err(LexError::UnknownRegister(reg.to_string()))?,
        };
        Ok((reg, buf))
    }
}

impl<'b> Lexable<'b> for Operation {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        use Operation as O;
        let (op, buf) = collect_while(buf, |c| c.is_alphabetic())?;
        let op = match op {
            "mov" => O::MOV,
            "lw" => O::LW,
            "sw" => O::SW,
            "push" => O::PUSH,
            "pop" => O::POP,
            "jnz" => O::JNZ,
            "in" => O::IN,
            "out" => O::OUT,
            "cmp" => O::CMP,
            "adc" => O::ADC,
            "sbb" => O::SBB,
            "or" => O::OR,
            "nor" => O::NOR,
            "and" => O::AND,
            "mb" => O::MB,
            _ => Err(LexError::UnknownSymbol(op.to_string()))?,
        };
        Ok((op, buf))
    }
}
