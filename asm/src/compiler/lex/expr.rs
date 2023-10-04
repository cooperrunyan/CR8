use failure::Fail;

use super::lexable::{
    collect_while, expect, ignore_whitespace, LexErrorKind, LexResult, Lexable,
    UnknownIdentifierError,
};
use super::CompilerContext;

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

#[derive(Debug, Clone)]
pub enum Expr<'e> {
    Literal(usize),
    Variable(&'e str),
    Expr {
        lhs: Box<Expr<'e>>,
        op: ExprOperation,
        rhs: Box<Expr<'e>>,
    },
}

impl<'e> Expr<'e> {
    pub fn resolve(self, ctx: &CompilerContext) -> Result<usize, ResolutionError> {
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
        let (lhs, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
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
pub enum ExprOperation {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
}

impl ExprOperation {
    pub fn to_expr<'e>(&self, lhs: Expr<'e>, rhs: Expr<'e>) -> Expr<'e> {
        Expr::Expr {
            lhs: Box::new(lhs),
            op: *self,
            rhs: Box::new(rhs),
        }
    }

    pub fn apply(self, lhs: usize, rhs: usize) -> Result<usize, ApplyError> {
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
