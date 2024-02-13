use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    marker::PhantomData,
    rc::Rc,
};

use parm_ast::parser::nodes::statement::use_stmt::{
    FunctionDeclaration, LetStatement, Param, StructDeclaration,
};

use crate::{
    ty::{Type, TypeRef},
    typechecker::Typechecker,
};

#[derive(Clone, PartialEq)]
pub struct Symbol<'a, 'b> {
    pub inner: Rc<RefCell<InnerSymbol<'a, 'b>>>,
}
impl<'a, 'b> std::fmt::Debug for Symbol<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.borrow().fmt(f)
    }
}
#[derive(Clone, PartialEq)]
pub struct InnerSymbol<'a, 'b> {
    pub id: usize,
    pub declaration: SymbolDeclaration<'a, 'b>,
    pub ty: TypeRef<'a, 'b>,
    pub lexeme: &'a str,
    pub tc: *const Typechecker<'a, 'b>,
}
impl<'a, 'b> std::fmt::Debug for InnerSymbol<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tc = unsafe { &*self.tc };
        f.debug_struct("Symbol")
            .field("id", &self.id)
            .field("declaration", &self.declaration)
            .field("ty", &tc.types_arena.types[self.ty.idx])
            .field("lexeme", &self.lexeme)
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
#[derive(Clone, PartialEq)]
pub enum SymbolDeclaration<'a, 'b> {
    Function(&'b FunctionDeclaration<'a>),
    Struct(&'b StructDeclaration<'a>),
    LetStatement(&'b LetStatement<'a>),
    Param(&'b Param<'a>),
    None,
}
impl<'a, 'b> InnerSymbol<'a, 'b> {
    pub fn get_ty(&self, typechecker: &Typechecker<'a, 'b>) -> Type<'a, 'b> {
        typechecker.types_arena.types[self.id].clone()
    }
}
impl<'a, 'b> Debug for SymbolDeclaration<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(arg0) => f.debug_tuple("Function").field(&arg0.name.lexeme).finish(),
            Self::Struct(arg0) => f.debug_tuple("Struct").field(&arg0.ident.lexeme).finish(),
            Self::LetStatement(arg0) => f
                .debug_tuple("LetStatement")
                .field(&arg0.ident.lexeme)
                .finish(),
            Self::Param(arg0) => f.debug_tuple("Param").field(&arg0.name).finish(),
            Self::None => write!(f, "None"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SymbolArena<'a, 'b> {
    pub symbols: Vec<Symbol<'a, 'b>>,
}
