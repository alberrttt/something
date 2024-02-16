use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use parm_ast::parser::nodes::statement::use_stmt::StructDeclaration as ASTStructDeclaration;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
}

impl<'a, 'b> Check<'a, 'b> for StructDeclaration<'a, 'b> {
    type Output = Self;

    type Ast = ASTStructDeclaration<'a>;

    fn check(tc: &mut Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output {
        let symbol = InnerSymbol {
            declaration: SymbolDeclaration::Struct(ast),
            ty: Type::None(PhantomData),
            lexeme: ast.ident.lexeme,
        };
        let symbol = Symbol {
            inner: Rc::new(RefCell::new(symbol)),
        };
        tc.mut_current_scope().push_symbol(symbol.clone());
        StructDeclaration { symbol }
    }
}
