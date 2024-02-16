use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{
    symbol::{self, InnerSymbol, Symbol},
    traits::Check,
    ty::Type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StructExpression<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub ast: &'b parm_ast::parser::nodes::expression::expr_struct::StructExpression<'a>,
}

impl<'a, 'b> Check<'a, 'b> for StructExpression<'a, 'b> {
    type Output = Self;

    type Ast = parm_ast::parser::nodes::expression::expr_struct::StructExpression<'a>;

    fn check(tc: &mut crate::typechecker::Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output {
        let symbol = InnerSymbol {
            declaration: crate::symbol::SymbolDeclaration::None,
            ty: Type::None(PhantomData),
            lexeme: ast.ident.lexeme,
        };
        let scope = tc.mut_current_scope();

        let symbol = Symbol {
            inner: Rc::new(RefCell::new(symbol)),
        };

        scope.push_symbol(symbol.clone());
        Self { symbol, ast }
    }
}
