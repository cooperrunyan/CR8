mod expr;
mod lexable;

use std::fmt::Debug;
use std::path::PathBuf;
use std::rc::Rc;

use failure::Fail;
use indexmap::IndexMap;

use crate::compiler::SourceInputError;
use crate::op::Operation;
use crate::reg::Register;

use self::expr::Expr;
use self::lexable::expect;

use super::Input;

use lexable::{
    collect_until, collect_while, expect_complete, ignore_whitespace, LexError, LexErrorKind,
    LexResult, Lexable, LexableWith, ParseRegisterError, UnknownIdentifierError,
};

#[derive(Debug, Default)]
pub struct CompilerContext<'c> {
    labels: IndexMap<String, usize>,
    statics: IndexMap<&'c str, usize>,
    vars: IndexMap<&'c str, usize>,
    macros: IndexMap<&'c str, Macro<'c>>,
    files: Vec<PathBuf>,
    mem_root: usize,
    boot_to: Option<&'c str>,
    nodes: Vec<Node<'c>>,
}

#[derive(Fail, Debug)]
pub enum CompileError {
    #[fail(display = "{}", _0)]
    LexError(#[cause] LexErrorKind),

    #[fail(display = "{}", _0)]
    SourceInputError(#[cause] SourceInputError),

    #[fail(display = "{}", _0)]
    Redefine(String),
}

enum Item<'i> {
    Meta(Directive<'i>),
    Node(Node<'i>),
}

#[derive(Debug)]
enum Node<'n> {
    Instruction(Instruction<'n>),
    Label(&'n str),
    Explicit(&'n str, ExplicitBytes),
    Import(Import<'n>),
}

impl<'b> Lexable<'b> for Item<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        if buf.starts_with("#") {
            let (dir, buf) = Directive::lex(buf)?;
            return Ok((Self::Meta(dir), buf));
        }

        if let Ok(_) = expect(buf, ".") {
            let (label, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_' || c == '.')?;
            let buf = ignore_whitespace(buf);
            let buf = expect(buf, ":")?;
            return Ok((Self::Node(Node::Label(label)), buf));
        }

        let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
        let buf = ignore_whitespace(buf);
        if let Ok(buf) = expect(buf, ":") {
            return Ok((Self::Node(Node::Label(id)), buf));
        }

        let (args, buf) = Vec::<Value>::lex(buf)?;
        Ok((Self::Node(Node::Instruction(Instruction { id, args })), buf))
    }
}

#[derive(Debug)]
struct Instruction<'i> {
    id: &'i str,
    args: Vec<Value<'i>>,
}

#[derive(Debug)]
enum Value<'v> {
    Expr(Expr<'v>),
    Immediate(usize),
    Register(Register),
    MacroVariable(&'v str),
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

        if let Ok(_) = expect(buf, "$") {
            let (var, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_' || c == '$')?;
            return Ok((Value::MacroVariable(var), buf));
        }

        let (val, buf) = usize::lex(buf)?;
        return Ok((Value::Immediate(val), buf));
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

#[derive(Debug)]
struct Macro<'m> {
    id: &'m str,
    captures: Vec<MacroCapture<'m>>,
}

#[derive(Debug)]
struct MacroCapture<'m> {
    args: Vec<MacroCaptureArg<'m>>,
    content: Vec<Instruction<'m>>,
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
        let (mut raw, buf) = collect_until(buf, |c| c == '}')?;
        let buf = expect(buf, "}")?;

        let mut content = vec![];

        loop {
            raw = ignore_whitespace(raw);
            if raw.is_empty() {
                break;
            }
            let (id, r) = collect_while(raw, |c| c.is_alphanumeric() || c == '_')?;
            raw = r;
            raw = ignore_whitespace(raw);
            let (args, r) = Vec::<Value>::lex(raw)?;
            raw = r;
            content.push(Instruction { id, args });
        }

        Ok((MacroCapture { args, content }, buf))
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

impl<'b> Lexable<'b> for CompilerContext<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        let mut ctx = CompilerContext::default();
        let mut buf = buf;

        loop {
            buf = ignore_whitespace(buf);
            if buf.is_empty() {
                break;
            }
            let (n, b) = Item::lex(buf)?;
            buf = b;
            match n {
                Item::Meta(d) => match d {
                    Directive::Boot(to) => {
                        if ctx.boot_to.is_some() {
                            return Err((LexErrorKind::Redefinition, "#[boot]"));
                        }
                        ctx.boot_to = Some(to);
                    }
                    Directive::DynOrigin(org) => {
                        if ctx.mem_root != 0 {
                            return Err((LexErrorKind::Redefinition, "#[dyn(&)]"));
                        }
                        ctx.mem_root = org;
                    }
                    Directive::ExplicitBytes(id, explicit) => {
                        ctx.nodes.push(Node::Explicit(id, explicit));
                    }
                    Directive::Macro(m) => {
                        if ctx.macros.contains_key(m.id) {
                            return Err((LexErrorKind::Redefinition, m.id));
                        }
                        ctx.macros.insert(m.id, m);
                    }
                    Directive::Static(id, val) => {
                        if ctx.statics.contains_key(id) {
                            return Err((LexErrorKind::Redefinition, id));
                        }
                        ctx.statics.insert(id, val);
                    }
                    Directive::Use(import) => ctx.nodes.push(Node::Import(import)),
                    Directive::Dyn(id, len) => {
                        if ctx.vars.contains_key(id) {
                            return Err((LexErrorKind::Redefinition, id));
                        }
                        ctx.vars.insert(id, len);
                    }
                },
                Item::Node(n) => ctx.nodes.push(n),
            }
        }

        Ok((ctx, buf))
    }
}

//////////////////////////////////

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
    fn d<'s>() -> Result<(), LexError<'s>> {
        let f = r#"
#[macro] nand: {
    ($into: reg, $rhs: imm8 | reg) => {
        and $into, $rhs
        not $into
    }
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        nand $inl, $frl
        nand $inh, $frh
    }
}

#[macro] not: {
    ($into: reg) => {
        nor $into, $into
    }
    ($inl: reg, $inh: reg) => {
        not $inl
        not $inh
    }
}

#[macro] xnor: {
    ($into: reg, $rhs: imm8 | reg) => {
        mov %f, $into
        nor $into, $rhs
        and %f, $rhs
        or $into, %f
    }
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        xnor $inl, $frl
        xnor $inh, $frh
    }
}


#[macro] xor: {
    ($into: reg, $rhs: imm8 | reg) => {
        mov %f, $rhs
        or %f, $into
        nand $into, $rhs
        and $into, %f
    }
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        xor $inl, $frl
        xor $inh, $frh
    }
}

#[static(ROM: 0x0000)]
#[static(BRAM: 0x8000)]
#[static(GPRAM: 0xC000)]
#[static(STACK: 0xFC00)]
#[static(STACK_END: 0xFEFF)]

#[static(PSR0: 0xFF00)]
#[static(PSR1: 0xFF01)]
#[static(PSR2: 0xFF02)]
#[static(PSR3: 0xFF03)]
#[static(PSR4: 0xFF04)]
#[static(PSR5: 0xFF05)]
#[static(PSR6: 0xFF06)]
#[static(PSR7: 0xFF07)]
#[static(PSR8: 0xFF08)]
#[static(PSR9: 0xFF09)]

#[static(CTRL: 0x00)]
#[static(SIGPING: 0x00)]
#[static(SIGHALT: 0x01)]
#[static(SIGDBG: 0x02)]
#[static(SIGBRKPT: 0x03)]

#[static(WAIT: 0x2000)]
#[static(OFFSET: 0x0400)]

#[boot]
main:
    mb 0x01
    jmp [hello]

hello:
    mov %a, %b, [HELLO]
    mov %c, %d, [BRAM]
    sw [PSR0], [HELLO_SZL]
    sw [PSR1], [HELLO_SZH]
    call [frmwof]
    wait [WAIT]
    clrvram [BRAM], [BRAM + HELLO_SZ]

    jmp [world]

world:
    mov %a, %b, [WORLD]
    mov %c, %d, [BRAM + OFFSET]
    sw [PSR0], [WORLD_SZL]
    sw [PSR1], [WORLD_SZH]
    call [frmwof]
    wait [WAIT]
    clrvram [BRAM + OFFSET], [BRAM + OFFSET + WORLD_SZ]

    jmp [hello]


        "#;

        let cc = CompilerContext::lex(f)?;

        dbg!(&cc);

        Ok(())
    }
}
