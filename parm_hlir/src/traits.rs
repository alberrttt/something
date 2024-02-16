use crate::typechecker::Typechecker;

pub trait Check<'a, 'b> {
    type Output;
    type Ast;
    fn check(tc: &mut Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output;
}
