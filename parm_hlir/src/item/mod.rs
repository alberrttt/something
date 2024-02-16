use self::function::Function;
use crate::prelude::*;
use crate::traits::Check;
use parm_ast::prelude::Item as ASTItem;

pub mod function;
pub mod struct_declaration;
#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a, 'b> {
    Function(Function<'a, 'b>),
    StructDeclaration(StructDeclaration<'a, 'b>),
}
impl<'a, 'b> Check<'a, 'b> for Item<'a, 'b> {
    type Output = Self;

    type Ast = ASTItem<'a>;

    fn check(tc: &mut crate::typechecker::Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output {
        match ast {
            ASTItem::Function(f) => return Item::Function(Function::check(tc, f)),
            ASTItem::Struct(s) => return Item::StructDeclaration(StructDeclaration::check(tc, s)),

            _ => {}
        };
        panic!()
    }
}
