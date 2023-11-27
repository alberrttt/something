use std::collections::HashMap;

use parmesan_ast::lexer::token::Ident;

#[derive(Debug, Clone, Default)]
pub struct LoweringContext<'a> {
    pub scopes: Vec<Scope<'a>>,
}

#[derive(Debug, Clone, Default)]
pub struct Scope<'a> {
    pub variables: Vec<Ident<'a>>,
}
pub trait Lower {
    type Output;

    fn parse(&self, ctx: &mut LoweringContext) -> Self::Output;
}
