use std::num::Wrapping;

use failure::Fail;

use crate::compiler::Compiler;

use super::lexable::{collect_while, expect, ignore_whitespace, LexError, LexResult, Lexable};

#[derive(Fail, Debug)]
pub enum ResolutionError {
    #[fail(display = "Unknown operation")]
    UnknownOperation,

    #[fail(display = "Unknown variable")]
    UnknownVariable,

    #[fail(display = "Operation failed")]
    OperationFailed,
}

#[derive(Fail, Debug)]
#[fail(display = "Operator application error")]
pub struct ApplyError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Literal(usize),
    Variable(String),
    Expr {
        lhs: Box<Expr>,
        op: ExprOperation,
        rhs: Box<Expr>,
    },
}

impl Expr {
    pub fn resolve(self, ctx: &Compiler) -> Result<usize, ResolutionError> {
        match self {
            Self::Literal(lit) => Ok(lit),
            Self::Variable(var) => {
                if var.as_str() == "$" {
                    Ok(ctx.pc)
                } else if let Some(label) = ctx.labels.get(&var) {
                    Ok(*label)
                } else if let Some(stat) = ctx.statics.get(&var) {
                    Ok(*stat)
                } else if let Some(label) = ctx.labels.get(&format!("{}{var}", &ctx.last_label)) {
                    Ok(*label)
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

    if let Ok(buf) = expect(buf, "$") {
        Ok((Expr::Variable("$".to_string()), buf))
    } else if let Ok((lhs, buf)) = usize::lex(buf) {
        Ok((Expr::Literal(lhs), buf))
    } else {
        let (lhs, buf) = collect_while(buf, |c| {
            c.is_alphanumeric() || c == '_' || c == '$' || c == '.'
        })?;
        Ok((Expr::Variable(lhs.to_string()), buf))
    }
}

fn lex_expr<'b>(buf: &'b str) -> LexResult<'b, Expr> {
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

impl<'b> Lexable<'b> for Expr {
    fn lex(buf: &'b str) -> LexResult<'b, Expr> {
        lex_expr(buf)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprOperation {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Rsh,
    Lsh,
}

impl ExprOperation {
    pub fn to_expr(&self, lhs: Expr, rhs: Expr) -> Expr {
        Expr::Expr {
            lhs: Box::new(lhs),
            op: *self,
            rhs: Box::new(rhs),
        }
    }

    pub fn apply(self, lhs: usize, rhs: usize) -> Result<usize, ApplyError> {
        match self {
            Self::Add => Ok((Wrapping(lhs) + Wrapping(rhs)).0),
            Self::Sub => Ok((Wrapping(lhs) - Wrapping(rhs)).0),
            Self::Mul => Ok((Wrapping(lhs) * Wrapping(rhs)).0),
            Self::Div => Ok((Wrapping(lhs) / Wrapping(rhs)).0),
            Self::And => Ok((Wrapping(lhs) & Wrapping(rhs)).0),
            Self::Or => Ok((Wrapping(lhs) | Wrapping(rhs)).0),
            Self::Rsh => Ok((Wrapping(lhs) >> rhs).0),
            Self::Lsh => Ok((Wrapping(lhs) << rhs).0),
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
        } else if let Ok(buf) = expect(buf, ">>") {
            (Self::Rsh, buf)
        } else if let Ok(buf) = expect(buf, "<<") {
            (Self::Lsh, buf)
        } else {
            Err(LexError::UnknownOperator(buf.to_string()))?
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex_expression() -> Result<(), Box<dyn std::error::Error>> {
        let ctx = Compiler::new();

        let (expr, _) = Expr::lex("1 + 0b01 + 2 * 3")?;
        let res = expr.resolve(&ctx).unwrap();
        assert_eq!(res, 1 + 0b01 + 2 * 3);

        let (expr, _) = Expr::lex("1 + (0b01 + 2) * 3")?;
        let res = expr.resolve(&ctx).unwrap();
        assert_eq!(res, 1 + (0b01 + 2) * 3);

        Ok(())
    }
}
