use std::{borrow::BorrowMut, cell::RefCell, marker::PhantomData, rc::Rc};

use crate::prelude::*;

use self::ty::struct_ty::StructTy;

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
}

impl<'a, 'b> Check<'a, 'b> for StructDeclaration<'a, 'b> {
    type Output = Self;

    type Ast = ast::StructDeclaration<'a>;

    fn check(tc: &mut Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output {
        let symbol = InnerSymbol {
            declaration: SymbolDeclaration::Struct(AST(ast)),
            ty: Type::None(PhantomData),
            lexeme: ast.ident.lexeme,
        };

        let symbol = Symbol {
            inner: Rc::new(RefCell::new(symbol)),
        };

        let mut fields = vec![];
        for field in &ast.body.elements() {
            let name = &field.ident;
            let ty = &field.ty;
            let ty: Type<'_, '_> = Type::check(tc, ty);
            let symbol = InnerSymbol {
                declaration: SymbolDeclaration::StructMemberDeclaration(AST(field)),
                ty,
                lexeme: name.lexeme,
            };
            let symbol = Symbol {
                inner: Rc::new(RefCell::new(symbol)),
            };
            fields.push(symbol);
        }
        let ty = StructTy {
            symbol: symbol.clone(),
            fields,
        };

        symbol.inner.as_ref().borrow_mut().ty = Type::Struct(Rc::new(ty));
        tc.mut_current_scope().push_symbol(symbol.clone());

        StructDeclaration { symbol }
    }
}
