use std::{
    cell::{RefCell, UnsafeCell},
    fmt::Debug,
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
    
    pub ty: TypeRef<'a>,
}
impl<'a> Symbol<'a> {
    pub fn new(name: &'a str, ty: TypeRef<'a>, source_file: *const SourceFile<'a>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerSymbol {
                source_file,
                name,
                ty,
            })),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolDeclaration<'a> {
    LetStatement(&'a LetStatement<'a>),
    FunctionDeclaration(&'a Function<'a>),
    Param(&'a Param<'a>),
}
