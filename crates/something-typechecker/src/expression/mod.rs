use std::rc::Rc;

use something_frontend::{block::Block, Expression, Ident};

use crate::{
    context::{BlockCtx, Scope},
    prelude::Type,
    symbol::Symbol,
    traits::ResolveType,
};

impl ResolveType<&mut BlockCtx> for Expression {
    fn resolve_type(&self, ctx: &mut BlockCtx) -> crate::prelude::Type {
        match self {
            Expression::Lit(lit) => Type::from(lit),
            Expression::Binary(bin) => Type::from(bin),
            Expression::Call(_) => todo!(),
            Expression::Ident(ident) => ident.resolve_type(ctx),
            Expression::Grouping(_) => todo!(),
            Expression::If(_) => todo!(),
            Expression::Block(_) => todo!(),
        }
    }
}
impl ResolveType<&mut BlockCtx> for Ident {
    fn resolve_type(&self, ctx: &mut BlockCtx) -> Type {
        ctx.get(Rc::new(Symbol::from(self))).unwrap()
    }
}
