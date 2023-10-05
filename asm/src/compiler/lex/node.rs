use crate::reg::Register;

use super::directive::{ExplicitBytes, Import};
use super::expr::Expr;
use super::lexable::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Node<'n> {
    Instruction(Instruction<'n>),
    Label(&'n str),
    Explicit(&'n str, ExplicitBytes),
    Import(Import<'n>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction<'i> {
    pub id: &'i str,
    pub args: Vec<Value<'i>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value<'v> {
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

#[cfg(test)]
mod test {
    use crate::compiler::lex::{ExprOperation, Item};

    use super::*;

    #[test]
    fn lex_instruction() -> Result<(), Box<dyn std::error::Error>> {
        let (n, _) = Item::lex("mov %c, %d, [BRAM + OFFSET]")?;
        assert_eq!(
            n,
            Item::Node(Node::Instruction(Instruction {
                id: "mov",
                args: vec![
                    Value::Register(Register::C),
                    Value::Register(Register::D),
                    Value::Expr(Expr::Expr {
                        lhs: Box::new(Expr::Variable("BRAM")),
                        op: ExprOperation::Add,
                        rhs: Box::new(Expr::Variable("OFFSET"))
                    })
                ]
            }))
        );

        Ok(())
    }
}
