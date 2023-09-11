use super::{ast::Ident, Compiler};

mod directives;
mod expr;
mod labels;
mod macros;

impl Compiler {
    pub(crate) fn resolve_static(&self, name: &str) -> Result<i128, ()> {
        let Some(stat) = self.statics.get(name) else {
            return Err(());
        };
        return Ok(stat.to_owned() as i128);
    }

    pub(crate) fn resolve_ident(&self, ident: &Ident) -> Result<i128, ()> {
        match ident {
            Ident::Addr(a) => self.resolve_expr(&a).map_err(|_| ()),
            Ident::Static(s) => self.resolve_static(&s),
            Ident::PC => Ok(self.pc as i128),
            _ => Err(()),
        }
    }
}
