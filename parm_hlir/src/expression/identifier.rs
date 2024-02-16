use crate::prelude::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub ast: AST<&'b ast::Identifier<'a>>,
}

impl<'a, 'b> Check<'a, 'b> for Identifier<'a, 'b> {
    type Output = Self;

    type Ast = ast::Identifier<'a>;

    fn check(tc: &mut crate::typechecker::Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output {
        let symbol = tc.get_symbol(ast.lexeme).unwrap();

        Self {
            symbol,
            ast: AST(ast),
        }
    }
}
