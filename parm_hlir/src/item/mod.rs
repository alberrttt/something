use self::function::Function;
use self::traits::TypeCheckResult;
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
impl<'a, 'b> Check<'a, 'b, Item<'a, 'b>> for ASTItem<'a> {
    fn check(
        &'b self,
        tc: &mut crate::typechecker::Typechecker<'a, 'b>,
    ) -> TypeCheckResult<'a, 'b, Item<'a, 'b>> {
        match self {
            ASTItem::Function(f) => return Ok(Item::Function(f.check(tc)?)),
            ASTItem::Struct(s) => return Ok(Item::StructDeclaration(s.check(tc)?)),
            
            _ => {}
        };
        panic!()
    }
}
