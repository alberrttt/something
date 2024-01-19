use std::collections::HashMap;

use parm_ast::prelude::*;
use parm_typechecker::Scope;

use crate::{RegIdx, Register, IR};

#[derive(Debug, Clone)]
pub struct IRScope<'a, 'b: 'a, 'c: 'a> {
    pub scope: &'c Scope<'a, 'b>,
    pub declaration: ScopeDeclaration<'a, 'c>,
    pub children: Vec<IRScope<'a, 'b, 'c>>,
    pub prologue: Vec<IR>,
    pub epilogue: Vec<IR>,
    pub variables: HashMap<&'b str, Location>,
}
#[derive(Debug, Clone)]
pub enum Location {
    Register(RegIdx),
    Stack(i32),
}
impl<'a, 'b, 'c: 'a> std::ops::Deref for IRScope<'a, 'b, 'c> {
    type Target = Scope<'a, 'b>;
    fn deref(&self) -> &Self::Target {
        self.scope
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeDeclaration<'a, 'b: 'a> {
    FunctionDeclaration(&'b Function<'a>),
    Block(&'b Block<'a>),
}
