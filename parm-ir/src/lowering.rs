use std::collections::HashMap;

use parm_ast::lexer::token::Ident;

#[derive(Debug, Clone, Default)]
pub struct LoweringContext<'a> {
    pub scopes: Vec<Scope<'a>>,
}
impl<'a> LoweringContext<'a> {
    pub fn top(&self) -> &Scope<'a> {
        &self.scopes.last().unwrap()
    }

    pub fn top_mut(&mut self) -> &mut Scope<'a> {
        self.scopes.last_mut().unwrap()
    }
}
#[derive(Debug, Clone, Default)]
pub struct Scope<'a> {
    pub variables: Vec<Ident<'a>>,
}
pub trait Lower<'a> {
    type Output;

    fn lower(&self, ctx: &mut LoweringContext<'a>) -> Self::Output;
}
