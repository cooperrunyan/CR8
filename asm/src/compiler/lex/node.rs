use crate::reg::Register;

use super::expr::Expr;
use super::lexable::*;
use super::meta::{Constant, Use};

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Instruction(Instruction),
    Label(String),
    Constant(String, Constant),
    Use(Use),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub id: String,
    pub args: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Expr(Expr),
    Literal(usize),
    Register(Register),
    MacroVariable(String),
}

impl<'b> Lexable<'b> for Value {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        if let Ok((lit, buf)) = usize::lex(buf) {
            return Ok((Value::Literal(lit), buf));
        }

        if buf.chars().nth(0) == Some('%') {
            let (reg, buf) = Register::lex(buf)?;
            return Ok((Value::Register(reg), buf));
        }

        if expect(buf, "$").is_ok() {
            let (var, buf) = collect_while(buf, |c| {
                c.is_alphanumeric() || c == '_' || c == '$' || c == '.'
            })?;
            return Ok((Value::MacroVariable(var.to_string()), buf));
        }
        let (expr_buf, buf) = collect_until(buf, |c| c == ',' || c == '\n')?;
        let (expr, expr_buf) = Expr::lex(expr_buf)?;
        let expr_buf = ignore_whitespace(expr_buf);
        expect_complete(expr_buf)?;

        Ok((Value::Expr(expr), buf))
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::sync::Arc;

    use crate::compiler::lex::{ExprOperation, Item, ItemInner};

    use super::*;

    #[test]
    fn lex_instruction() -> Result<(), Box<dyn std::error::Error>> {
        let (n, _) = Item::lex_with("mov %c, %d, BRAM + OFFSET", Arc::new(PathBuf::new()))?;
        assert_eq!(
            n.item,
            ItemInner::Node(Node::Instruction(Instruction {
                id: "mov".to_string(),
                args: vec![
                    Value::Register(Register::C),
                    Value::Register(Register::D),
                    Value::Expr(Expr::Expr {
                        lhs: Box::new(Expr::Variable("BRAM".to_string())),
                        op: ExprOperation::Add,
                        rhs: Box::new(Expr::Variable("OFFSET".to_string()))
                    })
                ]
            }))
        );

        Ok(())
    }
}
