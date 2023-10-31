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

#[macro_export]
macro_rules! lex_enum {
    ($buf:expr; $($m:literal => $variant:expr,)*) => {
        $crate::compiler::lex::lexable::expect_enum($buf, &[ $(($m, $variant),)* ])
    }
}

#[macro_export]
macro_rules! token {
    ($buf:ident $(; $ch:literal $(| $oth:literal)*)?) => {
        $crate::compiler::lex::lexable::collect_while($buf, |c| c.is_alphanumeric() $( || c == $ch  $(|| c == $oth  )*  )?  )
    }
}

#[macro_export]
macro_rules! surround_inline {
    ($start:literal $buf:ident $end:literal $inner:block ) => {{
        let $buf = $crate::compiler::lex::lexable::expect($buf, $start)?;
        let $buf = $crate::compiler::lex::lexable::ignore_whitespace_noline($buf);
        let (inner, $buf) = $inner;
        let $buf = $crate::compiler::lex::lexable::ignore_whitespace_noline($buf);
        let $buf = $crate::compiler::lex::lexable::expect($buf, $end)?;
        (inner, $buf)
    }};
}

#[macro_export]
macro_rules! repeated {
    ($start:literal $buf:ident $end:literal $inner:block) => {{
        let mut _items = vec![];
        let mut $buf = $crate::compiler::lex::lexable::expect($buf, $start)?;
        let $buf = loop {
            $buf = $crate::compiler::lex::lexable::ignore_whitespace($buf);
            if let Ok($buf) = $crate::compiler::lex::lexable::expect($buf, $end) {
                break $buf;
            }
            let (_inner, _b) = $inner;
            $buf = _b;
            _items.push(_inner);
        };
        (_items, $buf)
    }};
    ($start:literal $buf:ident $delimeter:literal $end:literal $inner:block) => {{
        let mut _items = vec![];
        let mut $buf = $crate::compiler::lex::lexable::expect($buf, $start)?;
        let $buf = loop {
            $buf = $crate::compiler::lex::lexable::ignore_whitespace($buf);
            if let Ok($buf) = $crate::compiler::lex::lexable::expect($buf, $end) {
                break $buf;
            }
            let (_inner, _b) = $inner;
            $buf = _b;
            _items.push(_inner);
            $buf = ignore_whitespace($buf);
            if let Ok(b) = expect($buf, $delimeter) {
                $buf = b;
                continue;
            }
            if let Ok(b) = expect($buf, $end) {
                $buf = b;
                break $buf;
            }
            bail!("Expected {} or {}\n\n{}", $delimeter, $end, $buf);
        };
        (_items, $buf)
    }};
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

pub fn expect_enum<'b, E: Clone + Copy>(
    buf: &'b str,
    variants: &[(&'static str, E)],
) -> Result<(E, &'b str)> {
    for (pat, variant) in variants {
        if let Ok(buf) = expect(buf, pat) {
            return Ok((*variant, buf));
        }
    }
    bail!(
        "Expected one of {:#?}, got {:#?}",
        variants
            .iter()
            .map(|(m, _)| *m)
            .collect::<Vec<_>>()
            .join("\", \""),
        buf.split_ascii_whitespace().next().unwrap_or_default()
    );
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
        let buf = expect(buf, "%")?;
        let (reg, buf) = collect_while(buf, |c| c.is_alphabetic())?;
        let reg = match Register::try_from(reg) {
            Ok(r) => r,
            Err(()) => bail!("Unknown register {reg:#?} at {buf:#?}"),
        };
        Ok((reg, buf))
    }
}

impl<'b> Lexable<'b> for Operation {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let (op, buf) = collect_while(buf, |c| c.is_alphabetic())?;
        let op = match Operation::try_from(op) {
            Ok(o) => o,
            Err(()) => bail!("Unknown operation {op:#?} at {buf:#?}"),
        };
        Ok((op, buf))
    }
}
