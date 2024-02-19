use crate::prelude::*;

use self::traits::TypeCheckResult;
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub ast: AST<&'b ast::Identifier<'a>>,
}

impl<'a, 'b> Check<'a, 'b, Identifier<'a, 'b>> for ast::Identifier<'a> {
    fn check(
        &'b self,
        tc: &mut crate::typechecker::Typechecker<'a, 'b>,
    ) -> TypeCheckResult<'a, 'b, Identifier<'a, 'b>> {
        let symbol = tc.get_symbol(self.lexeme).unwrap();

        Ok(Identifier {
            symbol,
            ast: AST(self),
        })
    }
}
