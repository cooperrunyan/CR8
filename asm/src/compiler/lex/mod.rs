use std::fmt::Debug;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::ptr;
use std::rc::Rc;

use failure::Fail;
use indexmap::IndexMap;
use serde::Serialize;

use crate::op::Operation;
use crate::reg::Register;

use super::Input;

#[derive(Debug, PartialEq, Fail)]
#[fail(display = "unknown identifier")]
struct UnknownIdentifierError;

#[derive(Debug, PartialEq, Fail)]
#[fail(display = "unknown register")]
struct ParseRegisterError;

#[derive(Debug, PartialEq, Fail)]
#[fail(display = "unknown register")]
struct UnexpectedTypeError;

#[derive(Debug, PartialEq, Fail)]
enum LexErrorKind {
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

type LexError<'b> = (LexErrorKind, &'b str);

type LexResult<'b, T> = Result<(T, &'b str), LexError<'b>>;

trait Lexable<'b>: Sized {
    fn lex(buf: &'b str) -> LexResult<'b, Self>;
}

trait LexableWith<'b, W>: Sized {
    fn lex_with(buf: &'b str, with: W) -> LexResult<'b, Self>;
}

impl<'b, T: Lexable<'b>, W> LexableWith<'b, W> for T {
    fn lex_with(buf: &'b str, _with: W) -> LexResult<'b, Self> {
        Self::lex(buf)
    }
}

#[derive(Debug, Default)]
struct CompilerContext<'c> {
    labels: IndexMap<&'c str, usize>,
    statics: IndexMap<&'c str, usize>,
    macros: IndexMap<&'c str, Macro<'c>>,
    files: Vec<PathBuf>,
    mem_root: usize,
    boot_to: Option<&'c str>,
}

#[derive(Fail, Debug)]
enum SourceFileError {
    #[fail(display = "bad path")]
    BadPath,

    #[fail(display = "file not found")]
    FileNotFound,

    #[fail(display = "module not found")]
    ModNotFound,
}

fn source_file(import: Import) -> Result<String, SourceFileError> {
    match import {
        Import::File(_f) => Ok("".into()),
        Import::Module(_f) => Ok("".into()),
    }
}

#[derive(Fail, Debug)]
enum CompileError {
    #[fail(display = "{}", _0)]
    LexError(#[cause] LexErrorKind),

    #[fail(display = "{}", _0)]
    SourceFileError(#[cause] SourceFileError),
}

enum Node<'n> {
    Meta(Directive<'n>),
    Label(usize),
    Instruction(Instruction<'n>),
}

struct Instruction<'i> {
    id: &'i str,
    args: Vec<Value<'i>>,
}

#[derive(Debug)]
enum Value<'v> {
    Expr(Expr<'v>),
    Immediate(usize),
    Register(Register),
}

impl<'b> Lexable<'b> for Value<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        if let Ok(buf) = expect(buf, "[") {
            let (expr, buf) = collect_until(buf, |c| c == ']')?;
            let buf = expect(buf, "]")?;
            let (expr, eb) = Expr::lex(expr)?;
            let eb = ignore_whitespace(eb);
            expect_complete(eb)?;
            return Ok((Value::Expr(expr), buf));
        }
        if buf.chars().nth(0) == Some('%') {
            let (reg, buf) = Register::lex(buf)?;
            return Ok((Value::Register(reg), buf));
        }

        let (val, buf) = usize::lex(buf)?;
        return Ok((Value::Immediate(val), buf));
    }
}

impl<'b> Lexable<'b> for Vec<Value<'b>> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let mut values = vec![];
        let mut buf = buf;
        let buf = loop {
            buf = buf.trim_start_matches(&[' ', '\t']);
            if let Ok(b) = expect(buf, "\n") {
                buf = b;
                break buf;
            }
            let (val, b) = Value::lex(buf)?;
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

fn expect<'b>(buf: &'b str, expect: &'static str) -> Result<&'b str, LexError<'b>> {
    if buf.starts_with(expect) {
        Ok(&buf[expect.len()..])
    } else {
        Err((LexErrorKind::Expected(expect), buf))
    }
}

fn ignore_whitespace<'b>(buf: &'b str) -> &'b str {
    buf.trim_start_matches(char::is_whitespace)
}

fn collect<'b, M: Fn(char) -> bool>(buf: &'b str, check: M) -> LexResult<'b, &'b str> {
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

fn collect_until<'b, M: Fn(char) -> bool>(buf: &'b str, check: M) -> LexResult<'b, &'b str> {
    collect(buf, check)
}

fn collect_while<'b, M: Fn(char) -> bool>(buf: &'b str, check: M) -> LexResult<'b, &'b str> {
    collect(buf, |ch| !check(ch))
}

fn expect_complete<'b>(buf: &'b str) -> LexResult<'b, ()> {
    let buf = ignore_whitespace(buf);
    if buf.len() != 0 {
        return Err((LexErrorKind::NoInput, buf));
    }
    Ok(((), buf))
}

#[derive(Debug)]
struct Macro<'m> {
    id: &'m str,
    captures: Vec<MacroCapture<'m>>,
}

#[derive(Debug)]
struct MacroCapture<'m> {
    args: Vec<MacroCaptureArg<'m>>,
    content: Vec<&'m str>,
}

#[derive(Debug)]
enum MacroCaptureArgType {
    Register,
    Imm8,
    Imm16,
    Imm8OrRegister,
}

#[derive(Debug)]
struct MacroCaptureArg<'m> {
    id: &'m str,
    ty: MacroCaptureArgType,
}

impl<'b> Lexable<'b> for usize {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        if let Ok(buf) = expect(buf, "0x") {
            let (num, buf) = collect_while(buf, |c| c.is_digit(16) || c == '_')?;
            let num: usize = usize::from_str_radix(&num.replace('_', ""), 16)
                .map_err(|e| (LexErrorKind::ParseInt { err: e, radix: 16 }, buf))?;
            Ok((num, buf))
        } else if let Ok(buf) = expect(buf, "0b") {
            let (num, buf) = collect_while(buf, |c| c.is_digit(2) || c == '_')?;
            let num: usize = usize::from_str_radix(&num.replace('_', ""), 2)
                .map_err(|e| (LexErrorKind::ParseInt { err: e, radix: 2 }, buf))?;
            Ok((num, buf))
        } else {
            let (num, buf) = collect_while(buf, |c| c.is_digit(10) || c == '_')?;
            let num: usize = usize::from_str_radix(&num.replace('_', ""), 10)
                .map_err(|e| (LexErrorKind::ParseInt { err: e, radix: 10 }, buf))?;
            Ok((num, buf))
        }
    }
}

type ExplicitBytes = Vec<u8>;

impl<'b> Lexable<'b> for ExplicitBytes {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let mut buf = expect(buf, "{")?;
        let mut bytes = Self::new();
        loop {
            buf = ignore_whitespace(buf);
            if let Ok(buf) = expect(buf, "}") {
                return Ok((bytes, buf));
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
                return Ok((bytes, buf));
            }
            Err((LexErrorKind::Expected(","), buf))?;
        }
    }
}

#[derive(Debug)]
enum Directive<'d> {
    Boot(&'d str),
    ExplicitBytes(&'d str, ExplicitBytes),
    Dyn(&'d str, usize),
    DynOrigin(usize),
    Macro(Macro<'d>),
    Static(&'d str, usize),
    Use(Import<'d>),
}

#[derive(Debug)]
enum Import<'d> {
    File(&'d str),
    Module(&'d str),
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
            _ => Err((LexErrorKind::UnknownIdentifier(UnknownIdentifierError), buf)),
        }
    }
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

impl<'b> Lexable<'b> for Macro<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace(buf);
        let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
        let mut mac = Macro {
            id,
            captures: vec![],
        };

        let buf = ignore_whitespace(buf);
        let buf = expect(buf, ":")?;
        let buf = ignore_whitespace(buf);
        let buf = expect(buf, "{")?;
        let mut buf = buf;

        loop {
            buf = ignore_whitespace(buf);
            if let Ok(buf) = expect(buf, "}") {
                return Ok((mac, buf));
            }

            let (cap, b) = MacroCapture::lex(buf)?;
            buf = b;
            mac.captures.push(cap);
        }
    }
}

impl<'b> Lexable<'b> for MacroCapture<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let mut buf = expect(buf, "(")?;
        let mut args = vec![];

        loop {
            buf = ignore_whitespace(buf);
            if let Ok(b) = expect(buf, ")") {
                buf = b;
                break;
            }

            let (arg, b) = MacroCaptureArg::lex(buf)?;
            buf = b;

            args.push(arg);
            buf = ignore_whitespace(buf);
            if let Ok(b) = expect(buf, ",") {
                buf = b;
                continue;
            }
            if let Ok(b) = expect(buf, ")") {
                buf = b;
                break;
            }
            Err((LexErrorKind::Expected(","), buf))?;
        }

        let buf = ignore_whitespace(buf);
        let buf = expect(buf, "=>")?;
        let buf = ignore_whitespace(buf);
        let buf = expect(buf, "{")?;
        let buf = ignore_whitespace(buf);
        let (content, buf) = collect_until(buf, |c| c == '}')?;
        let buf = expect(buf, "}")?;

        Ok((
            MacroCapture {
                args,
                content: content.lines().collect::<Vec<_>>(),
            },
            buf,
        ))
    }
}

impl<'b> Lexable<'b> for MacroCaptureArg<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_' || c == '$')?;
        let buf = ignore_whitespace(buf);
        let buf = expect(buf, ":")?;
        let buf = ignore_whitespace(buf);
        let (ty, buf) = MacroCaptureArgType::lex(buf)?;

        Ok((MacroCaptureArg { id, ty }, buf))
    }
}

impl<'b> Lexable<'b> for MacroCaptureArgType {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let buf = ignore_whitespace(buf);
        let (ty0, buf) = collect_while(buf, |c| c.is_alphanumeric())?;
        let buf = ignore_whitespace(buf);
        if let Ok(buf) = expect(buf, "|") {
            let buf = ignore_whitespace(buf);
            let (ty1, buf) = collect_while(buf, |c| c.is_alphanumeric())?;
            if (ty0 == "reg" && ty1 == "imm8") || (ty1 == "reg" && ty0 == "imm8") {
                return Ok((Self::Imm8OrRegister, buf));
            }
            return Err((LexErrorKind::Expected("macro arg type"), buf));
        }

        Ok((
            match ty0 {
                "reg" => Self::Register,
                "imm8" => Self::Imm8,
                "imm16" => Self::Imm16,
                _ => Err((LexErrorKind::ArgType("Unknown arg type"), buf))?,
            },
            buf,
        ))
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
            _ => Err((LexErrorKind::ParseRegister(ParseRegisterError), buf))?,
        };
        Ok((reg, buf))
    }
}

impl<'b> Lexable<'b> for Operation {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        use Operation as O;
        let (reg, buf) = collect_while(buf, |c| c.is_alphabetic())?;
        let reg = match reg {
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
            _ => Err((LexErrorKind::ParseRegister(ParseRegisterError), buf))?,
        };
        Ok((reg, buf))
    }
}

//////////////////////////////////

#[derive(Fail, Debug)]
enum ResolutionError {
    #[fail(display = "Unknown operation")]
    UnknownOperation,

    #[fail(display = "Unknown variable")]
    UnknownVariable,

    #[fail(display = "Operation failed")]
    OperationFailed,
}

#[derive(Fail, Debug)]
#[fail(display = "Operator application error")]
struct ApplyError;

trait Applicable: Debug {
    fn apply(self, lhs: usize, rhs: usize) -> Result<usize, ApplyError>;
}

#[derive(Debug, Clone)]
enum Expr<'e> {
    Literal(usize),
    Variable(&'e str),
    Expr {
        lhs: Box<Expr<'e>>,
        op: ExprOperation,
        rhs: Box<Expr<'e>>,
    },
}

impl<'e> Expr<'e> {
    fn resolve(self, ctx: &CompilerContext) -> Result<usize, ResolutionError> {
        match self {
            Self::Literal(lit) => Ok(lit),
            Self::Variable(var) => {
                if let Some(label) = ctx.labels.get(var) {
                    Ok(*label)
                } else if let Some(stat) = ctx.statics.get(var) {
                    Ok(*stat)
                } else {
                    Err(ResolutionError::UnknownVariable)
                }
            }
            Self::Expr { lhs, op, rhs } => op
                .apply(lhs.resolve(ctx)?, rhs.resolve(ctx)?)
                .map_err(|_| ResolutionError::OperationFailed),
        }
    }
}

fn lex_expr_lhs<'b>(buf: &'b str) -> LexResult<'b, Expr> {
    let buf = ignore_whitespace(buf);

    if let Ok(buf) = expect(buf, "(") {
        let buf = ignore_whitespace(buf);
        let (ex, buf) = Expr::lex(buf)?;
        let buf = ignore_whitespace(buf);
        let buf = expect(buf, ")")?;
        return Ok((ex, buf));
    }

    if let Ok((lhs, buf)) = usize::lex(buf) {
        Ok((Expr::Literal(lhs), buf))
    } else {
        let (lhs, buf) = collect_while(buf, |c| c.is_alphabetic() || c == '_')?;
        Ok((Expr::Variable(lhs), buf))
    }
}

impl<'b> Lexable<'b> for Expr<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Expr<'b>> {
        let (lhs, buf) = lex_expr_lhs(buf)?;
        let buf = ignore_whitespace(buf);

        if let Ok((op, buf)) = ExprOperation::lex(buf) {
            let buf = ignore_whitespace(buf);
            if op == ExprOperation::Mul || op == ExprOperation::Div {
                let (rhs, buf) = lex_expr_lhs(buf)?;
                let buf = ignore_whitespace(buf);

                let lhs = op.to_expr(lhs, rhs);

                if let Ok((next_op, buf)) = ExprOperation::lex(buf) {
                    let buf = ignore_whitespace(buf);

                    let (rhs, buf) = Expr::lex(buf)?;

                    return Ok((next_op.to_expr(lhs, rhs), buf));
                } else {
                    return Ok((lhs, buf));
                }
            } else {
                let buf = ignore_whitespace(buf);

                let (rhs, buf) = Expr::lex(buf)?;

                return Ok((op.to_expr(lhs, rhs), buf));
            };
        } else {
            Ok((lhs, buf))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExprOperation {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
}

impl ExprOperation {
    fn to_expr<'e>(&self, lhs: Expr<'e>, rhs: Expr<'e>) -> Expr<'e> {
        Expr::Expr {
            lhs: Box::new(lhs),
            op: *self,
            rhs: Box::new(rhs),
        }
    }
}

impl Applicable for ExprOperation {
    fn apply(self, lhs: usize, rhs: usize) -> Result<usize, ApplyError> {
        match self {
            Self::Add => Ok(lhs + rhs),
            Self::Sub => Ok(lhs - rhs),
            Self::Mul => Ok(lhs * rhs),
            Self::Div => Ok(lhs / rhs),
            Self::And => Ok(lhs & rhs),
            Self::Or => Ok(lhs | rhs),
        }
    }
}

impl<'b> Lexable<'b> for ExprOperation {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        Ok(if let Ok(buf) = expect(buf, "*") {
            (Self::Mul, buf)
        } else if let Ok(buf) = expect(buf, "+") {
            (Self::Add, buf)
        } else if let Ok(buf) = expect(buf, "-") {
            (Self::Sub, buf)
        } else if let Ok(buf) = expect(buf, "/") {
            (Self::Div, buf)
        } else if let Ok(buf) = expect(buf, "&") {
            (Self::And, buf)
        } else if let Ok(buf) = expect(buf, "|") {
            (Self::Or, buf)
        } else {
            Err((LexErrorKind::UnknownIdentifier(UnknownIdentifierError), buf))?
        })
    }
}

#[cfg(test)]
mod test {
    use super::{CompilerContext, Directive, Expr, LexError, Lexable};
    use indexmap::IndexMap;

    #[test]
    fn directive<'s>() -> Result<(), LexError<'s>> {
        let _ = Directive::lex(
            r#"#[macro] jnz: {
                ($addr: imm16, $if: imm8 | reg) => {
                    ldhl $addr
                    jnz $if
                }
                ($addr: imm8, $if: imm8 | reg) => {
                    jnz $if
                }
            }"#,
        )?;

        let _ = Directive::lex("#[static(HELLO: 0xFF00)]")?;
        let _ = Directive::lex("#[static(HELLO: 2)]")?;
        let _ = Directive::lex("#[static(HELLO: 0b1001)]")?;

        let _ = Directive::lex("#[use(\"./std/test.asm\")]")?;
        let _ = Directive::lex("#[use(std::test)]")?;

        let _ = Directive::lex("#[dyn(TEST: 4)]")?;
        let _ = Directive::lex("#[dyn(&0xC000)]")?;

        let _ = Directive::lex("#[boot] main: mov %a, %b")?;

        let _ = Directive::lex(
            r#"#[explicit(TEST)] {
                    0x00, 0x00, 0x00
                }"#,
        )?;
        Ok(())
    }

    #[test]
    fn expr<'s>() -> Result<(), LexError<'s>> {
        let ctx = CompilerContext::default();
        assert!(Expr::lex("1 + 2")?.0.resolve(&ctx).unwrap() == 3);
        assert!(Expr::lex("1 + 2 * 3")?.0.resolve(&ctx).unwrap() == 7);
        assert!(Expr::lex("(1 + 2) * 3")?.0.resolve(&ctx).unwrap() == 9);

        let mut ctx = CompilerContext {
            statics: IndexMap::new(),
            ..Default::default()
        };

        ctx.statics.insert("A", 1);
        ctx.statics.insert("B", 2);
        ctx.statics.insert("C", 3);

        assert!(Expr::lex("A + 2")?.0.resolve(&ctx).unwrap() == 3);
        assert!(Expr::lex("1 + B * 3")?.0.resolve(&ctx).unwrap() == 7);
        assert!(Expr::lex("(1 + 2) * C")?.0.resolve(&ctx).unwrap() == 9);

        Ok(())
    }
}
