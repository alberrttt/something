use crate::{
    error::{TypeError, TypeErrorKind},
    prelude::*,
};

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
        match tc.get_symbol(self.lexeme) {
            Some(symbol) => Ok(Identifier {
                symbol,
                ast: AST(self),
            }),
            None => {
                tc.errs.push(TypeError::new(
                    TypeErrorKind::SymbolNotFound {
                        name: self.lexeme,
                        location: self.span,
                    },
                    tc.source_file.preparsed,
                ));
                Ok(Identifier {
                    symbol: tc.none_symbol.clone(),
                    ast: AST(self),
                })
            }
        }
    }
}
