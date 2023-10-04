use failure::Fail;
use std::fmt::Debug;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Fail)]
#[fail(display = "unknown identifier")]
pub struct UnknownIdentifierError;

#[derive(Debug, PartialEq, Fail)]
#[fail(display = "unknown register")]
pub struct ParseRegisterError;

#[derive(Debug, PartialEq, Fail)]
#[fail(display = "unknown register")]
pub struct UnexpectedTypeError;

#[derive(Debug, PartialEq, Fail)]
pub enum LexErrorKind {
    #[fail(display = "expected {}", _0)]
    Expected(&'static str),

    #[fail(display = "{} parsing (radix: {})", err, radix)]
    ParseInt {
        #[cause]
        err: ParseIntError,
        radix: u32,
    },

    #[fail(display = "{}", _0)]
    ParseRegister(#[cause] ParseRegisterError),

    #[fail(display = "{}", _0)]
    UnknownIdentifier(#[cause] UnknownIdentifierError),

    #[fail(display = "unexpected end of file")]
    EOF,

    #[fail(display = "unexpected no more input")]
    NoInput,

    #[fail(display = "unexpected redefinition")]
    Redefinition,

    #[fail(display = "invalid argument amount (expected {})", _0)]
    ArgAmt(usize),

    #[fail(display = "type error {}", _0)]
    ArgType(&'static str),
}

pub type LexError<'b> = (LexErrorKind, &'b str);

pub type LexResult<'b, T> = Result<(T, &'b str), LexError<'b>>;

pub trait Lexable<'b>: Sized {
    fn lex(buf: &'b str) -> LexResult<'b, Self>;
}

pub trait LexableWith<'b, W>: Sized {
    fn lex_with(buf: &'b str, with: W) -> LexResult<'b, Self>;
}

impl<'b, T: Lexable<'b>, W> LexableWith<'b, W> for T {
    fn lex_with(buf: &'b str, _with: W) -> LexResult<'b, Self> {
        Self::lex(buf)
    }
}

pub fn ignore_whitespace<'b>(buf: &'b str) -> &'b str {
    buf.trim_start_matches(char::is_whitespace)
}

pub fn collect<'b, M: Fn(char) -> bool>(buf: &'b str, check: M) -> LexResult<'b, &'b str> {
    for (i, ch) in buf.chars().enumerate() {
        if check(ch) {
            let remaining = &buf[i..];
            if i == 0 {
                return Err((LexErrorKind::Expected(""), buf));
            }
            return if remaining.len() == 0 {
                Err((LexErrorKind::EOF, buf))
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
        return Err((LexErrorKind::NoInput, buf));
    }
    Ok(((), buf))
}

pub fn expect<'b>(buf: &'b str, expect: &'static str) -> Result<&'b str, LexError<'b>> {
    if buf.starts_with(expect) {
        Ok(&buf[expect.len()..])
    } else {
        Err((LexErrorKind::Expected(expect), buf))
    }
}
