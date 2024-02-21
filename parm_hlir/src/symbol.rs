use std::{
    cell::{LazyCell, Ref, RefCell},
    fmt::Debug,
    marker::PhantomData,
    rc::Rc,
};

use parm_ast::parser::nodes::{
    declaration::struct_dec::StructMemberDeclaration,
    statement::use_stmt::{FunctionDeclaration, LetStatement, Param, StructDeclaration},
};

use crate::{ty::Type, typechecker::Typechecker, AST};

#[derive(Clone, PartialEq)]
pub struct Symbol<'a, 'b> {
    pub inner: Rc<RefCell<InnerSymbol<'a, 'b>>>,
}
impl<'a, 'b> Symbol<'a, 'b> {
    pub fn ty(&self) -> Type<'a, 'b> {
        self.inner.borrow().ty.clone()
    }
    pub fn new(decl: SymbolDeclaration<'a, 'b>, ty: Type<'a, 'b>, lexeme: &'a str) -> Self {
        Symbol {
            inner: Rc::new(RefCell::new(InnerSymbol {
                declaration: decl,
                ty,
                lexeme,
            })),
        }
    }
    pub fn set_ty(&self, ty: Type<'a, 'b>) {
        self.inner.borrow_mut().ty = ty;
    }
}

impl<'a, 'b> std::fmt::Debug for Symbol<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.borrow().fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct InnerSymbol<'a, 'b> {
    pub declaration: SymbolDeclaration<'a, 'b>,
    pub ty: Type<'a, 'b>,
    pub lexeme: &'a str,
}
impl<'a, 'b> std::fmt::Debug for InnerSymbol<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Symbol<{}>", self.lexeme))
            .field("ty", &self.ty)
            .field("declaration", &self.declaration)
            .finish()
    }
}
impl<'a, 'b> InnerSymbol<'a, 'b> {
    pub fn into_symbol(self) -> Symbol<'a, 'b> {
        Symbol {
            inner: Rc::new(RefCell::new(self)),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum SymbolDeclaration<'a, 'b> {
    Function(AST<&'b FunctionDeclaration<'a>>),
    Struct(AST<&'b StructDeclaration<'a>>),
    LetStatement(AST<&'b LetStatement<'a>>),
    StructMemberDeclaration(AST<&'b StructMemberDeclaration<'a>>),
    Param(AST<&'b Param<'a>>),
    None,
}
