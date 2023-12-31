use std::num::Wrapping;

use anyhow::{bail, Result};

use crate::compiler::Compiler;
use crate::{lex_enum, token};

use super::lexable::{expect, ignore_whitespace, LexResult, Lexable};

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
    pub fn resolve(&self, ctx: &Compiler) -> Result<usize> {
        match self {
            Self::Literal(lit) => Ok(*lit),
            Self::Variable(var) => Ok(if var.as_str() == "$" {
                ctx.bin.len()
            } else if let Some(label) = ctx.labels.get(var) {
                *label
            } else if let Some(stat) = ctx.statics.get(var) {
                *stat
            } else if let Some(label) = ctx.labels.get(&format!("{}{var}", &ctx.last_label)) {
                *label
            } else if let Some(d) = ctx.ram_locations.get(var) {
                *d
            } else {
                bail!("Unknown variable: {var:#?}");
            }),
            Self::Expr { lhs, op, rhs } => Ok(op.apply(lhs.resolve(ctx)?, rhs.resolve(ctx)?)?),
        }
    }
}

fn lex_expr_lhs(buf: &str) -> LexResult<'_, Expr> {
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
        let (lhs, buf) = token!(buf; '_' | '$' | '.')?;
        Ok((Expr::Variable(lhs.to_string()), buf))
    }
}

fn lex_expr(buf: &str) -> LexResult<'_, Expr> {
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

                Ok((next_op.to_expr(lhs, rhs), buf))
            } else {
                Ok((lhs, buf))
            }
        } else {
            let buf = ignore_whitespace(buf);

            let (rhs, buf) = Expr::lex(buf)?;

            Ok((op.to_expr(lhs, rhs), buf))
        }
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
    Xor,
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

    pub fn apply(self, lhs: usize, rhs: usize) -> Result<usize> {
        match self {
            Self::Add => Ok((Wrapping(lhs) + Wrapping(rhs)).0),
            Self::Sub => Ok((Wrapping(lhs) - Wrapping(rhs)).0),
            Self::Mul => Ok((Wrapping(lhs) * Wrapping(rhs)).0),
            Self::Div => Ok((Wrapping(lhs) / Wrapping(rhs)).0),
            Self::And => Ok((Wrapping(lhs) & Wrapping(rhs)).0),
            Self::Xor => Ok((Wrapping(lhs) ^ Wrapping(rhs)).0),
            Self::Or => Ok((Wrapping(lhs) | Wrapping(rhs)).0),
            Self::Rsh => Ok((Wrapping(lhs) >> rhs).0),
            Self::Lsh => Ok((Wrapping(lhs) << rhs).0),
        }
    }
}

impl<'b> Lexable<'b> for ExprOperation {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        lex_enum! { buf;
            "*" => Self::Mul,
            "+" => Self::Add,
            "-" => Self::Sub,
            "/" => Self::Div,
            "&" => Self::And,
            "^" => Self::Xor,
            "|" => Self::Or,
            ">>" => Self::Rsh,
            "<<" => Self::Lsh,
        }
        .map_err(|e| e.context("Unknown Operator"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex_expression() -> Result<(), Box<dyn std::error::Error>> {
        let ctx = Compiler::default();

        let (expr, _) = Expr::lex("1 + 0b01 + 2 * 3")?;
        let res = expr.resolve(&ctx).unwrap();
        assert_eq!(res, 1 + 0b01 + 2 * 3);

        let (expr, _) = Expr::lex("0xC011 & 0xFF")?;
        let res = expr.resolve(&ctx).unwrap();
        assert_eq!(res, 0xC011 & 0xFF);

        let (expr, _) = Expr::lex("0xC011 >> 8")?;
        let res = expr.resolve(&ctx).unwrap();
        assert_eq!(res, 0xC011 >> 8);

        let (expr, _) = Expr::lex("1 + (0b01 + 2) * 3")?;
        let res = expr.resolve(&ctx).unwrap();
        assert_eq!(res, 1 + (0b01 + 2) * 3);

        Ok(())
    }
}
