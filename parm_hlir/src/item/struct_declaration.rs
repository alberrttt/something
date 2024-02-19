use std::{borrow::BorrowMut, cell::RefCell, marker::PhantomData, rc::Rc};

use crate::prelude::*;

use self::{traits::TypeCheckResult, ty::struct_ty::StructTy};

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
}

impl<'a, 'b> Check<'a, 'b, StructDeclaration<'a, 'b>> for ast::StructDeclaration<'a> {
    fn check(
        &'b self,
        tc: &mut Typechecker<'a, 'b>,
    ) -> TypeCheckResult<'a, 'b, StructDeclaration<'a, 'b>> {
        let symbol = InnerSymbol {
            declaration: SymbolDeclaration::Struct(AST(self)),
            ty: Type::None(PhantomData),
            lexeme: self.ident.lexeme,
        };

        let symbol = Symbol {
            inner: Rc::new(RefCell::new(symbol)),
        };

        let mut fields = vec![];
        for field in &self.body.elements() {
            let name = &field.ident;
            let ty = &field.ty;
            let ty: Type<'_, '_> = ty.check(tc)?;
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

        Ok(StructDeclaration { symbol })
    }
}
