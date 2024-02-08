use std::{
    borrow::Borrow,
    cell::{Ref, RefCell, UnsafeCell},
    fmt::Debug,
    ops::Deref,
    rc::Rc,
};

use crate::ast::prelude::{Function, LetStatement, Param, SourceFile};

use super::ty::TypeRef;

#[derive(Clone, PartialEq)]
pub struct Symbol<'a> {
    pub inner: Rc<RefCell<InnerSymbol<'a>>>,
}
unsafe impl Sync for Symbol<'_> {}
impl<'a> Debug for Symbol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.inner)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InnerSymbol<'a> {
    // lol , this will hopeefully not be a problem
    pub source_file: *const SourceFile<'a>,
    pub name: &'a str,
    pub declaration: Option<SymbolDeclaration<'a>>,
    pub ty: TypeRef<'a>,
}
impl<'a> InnerSymbol<'a> {
    pub fn represents_value(&self) -> bool {
        matches!(
            self.declaration,
            Some(SymbolDeclaration::LetStatement(_)) | Some(SymbolDeclaration::Param(_))
        )
    }
}

impl<'a> Symbol<'a> {
    pub fn ref_inner<'b: 'a>(&'b self) -> Ref<'b, InnerSymbol<'a>> {
        let tmp: &RefCell<InnerSymbol<'_>> = self.inner.borrow();
        tmp.borrow()
    }
    pub fn new(name: &'a str, ty: TypeRef<'a>, source_file: *const SourceFile<'a>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerSymbol {
                source_file,
                name,
                ty,
                declaration: None,
            })),
        }
    }
    pub fn from_declaration(
        name: &'a str,
        ty: TypeRef<'a>,
        source_file: *const SourceFile<'a>,
        declaration: SymbolDeclaration<'a>,
    ) -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerSymbol {
                source_file,
                name,
                ty,
                declaration: Some(declaration),
            })),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolDeclaration<'a> {
    LetStatement(*const LetStatement<'a>),
    FunctionDeclaration(*const Function<'a>),
    Param(*const Param<'a>),
}
