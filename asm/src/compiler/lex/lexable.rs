use crate::op::Operation;
use crate::reg::Register;

use anyhow::{bail, Result};

pub type LexResult<'b, T> = Result<(T, &'b str)>;

pub trait Lexable<'b>: Sized {
    fn lex(buf: &'b str) -> LexResult<'b, Self>;
}

pub trait LexableWith<'b, W>: Sized {
    fn lex_with(buf: &'b str, with: W) -> LexResult<'b, Self>;
}

impl<'b, W, T: Lexable<'b>> LexableWith<'b, W> for T {
    fn lex_with(buf: &'b str, _with: W) -> LexResult<'b, Self> {
        T::lex(buf)
    }
}

pub fn ignore_comment(buf: &str) -> &str {
    if buf.starts_with(';') {
        if let Some(nl) = buf.find('\n') {
            ignore_comment(buf[nl..].trim_start_matches(char::is_whitespace))
        } else {
            ""
        }
    } else {
        buf
    }
}

pub fn ignore_whitespace(buf: &str) -> &str {
    let buf = buf.trim_start_matches(char::is_whitespace);
    let buf = ignore_comment(buf);
    let buf = buf.trim_start_matches(char::is_whitespace);
    buf
}

pub fn ignore_whitespace_noline(buf: &str) -> &str {
    buf.trim_start_matches([' ', '\t'])
}

pub fn collect<M: Fn(char) -> bool>(buf: &str, check: M) -> LexResult<'_, &str> {
    for (i, ch) in buf.chars().enumerate() {
        if check(ch) {
            let remaining = &buf[i..];
            if i == 0 {
                bail!(
                    "Unexpected {:#?}",
                    buf.split_ascii_whitespace().next().unwrap_or_default()
                );
            }
            return if remaining.is_empty() {
                bail!("Unexpected end of input");
            } else {
                Ok(buf.split_at(i))
            };
        }
    }

    Ok((buf, ""))
}

pub fn collect_until<M: Fn(char) -> bool>(buf: &str, check: M) -> LexResult<'_, &str> {
    collect(buf, check)
}

pub fn collect_while<M: Fn(char) -> bool>(buf: &str, check: M) -> LexResult<'_, &str> {
    collect(buf, |ch| !check(ch))
}

pub fn expect_complete(buf: &str) -> LexResult<'_, ()> {
    let buf = ignore_whitespace(buf);
    if !buf.is_empty() {
        bail!("Unexpected {:#?}", buf);
    }
    Ok(((), buf))
}

pub fn expect<'b>(buf: &'b str, expect: &'static str) -> Result<&'b str> {
    if let Some(buf) = buf.strip_prefix(expect) {
        Ok(buf)
    } else {
        bail!(
            "Expected {expect:#?}, got {:#?}",
            buf.split_ascii_whitespace().next().unwrap_or_default()
        );
    }
}

impl<'b, T: Lexable<'b>> Lexable<'b> for Vec<T> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let mut values = vec![];
        let mut buf = buf;
        let buf = loop {
            buf = buf.trim_start_matches([' ', '\t']);
            if buf.is_empty() {
                break buf;
            }
            if let Ok(b) = expect(buf, "\n") {
                buf = b;
                break buf;
            }
            let (val, b) = T::lex(buf)?;
            buf = b;
            buf = buf.trim_start_matches([' ', '\t']);
            if buf.is_empty() {
                break buf;
            }
            values.push(val);
            if let Ok(b) = expect(buf, ",") {
                buf = b;
                continue;
            }

            break buf;
        };
        Ok((values, buf))
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
        match usize::from_str_radix(num, radix) {
            Ok(val) => Ok((val, buf)),
            Err(_) => bail!("Failed to parse number {num:#?}"),
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
            _ => bail!("Unknown register {reg:#?} at {buf:#?}"),
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
            _ => bail!("Unknown operation {op:#?} at {buf:#?}"),
        };
        Ok((op, buf))
    }
}