use std::rc::Rc;

use crate::ast::prelude::SourceFile;

use super::ty::TypeRef;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol<'a> {
    pub inner: Rc<InnerSymbol<'a>>,
}
impl<'a> std::ops::Deref for Symbol<'a> {
    type Target = InnerSymbol<'a>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct InnerSymbol<'a> {
    // lol , this will hopeefully not be a problem
    pub source_file: *const SourceFile<'a>,
    pub name: &'a str,

    pub ty: TypeRef<'a>,
}
