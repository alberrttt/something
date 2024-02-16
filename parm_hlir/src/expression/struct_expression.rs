use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{
    symbol::{self, InnerSymbol, Symbol, SymbolDeclaration},
    traits::Check,
    ty::Type,
    AST,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StructExpression<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub ast: AST<&'b parm_ast::parser::nodes::expression::expr_struct::StructExpression<'a>>,
}

impl<'a, 'b> Check<'a, 'b> for StructExpression<'a, 'b> {
    type Output = Self;

    type Ast = parm_ast::parser::nodes::expression::expr_struct::StructExpression<'a>;

    fn check(tc: &mut crate::typechecker::Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output {
        let symbol = tc.get_symbol(ast.ident.lexeme).unwrap();
        let SymbolDeclaration::Struct(AST(_)) = symbol.inner.borrow().declaration else {
            panic!(
                "Expected struct symbol, found {:#?}",
                symbol.inner.borrow().declaration
            )
        };
        
        Self {
            symbol,
            ast: AST(ast),
        }
    }
}
