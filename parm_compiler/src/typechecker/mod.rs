use std::{
    cell::{RefCell, UnsafeCell},
    rc::Rc,
    slice::Windows,
};
pub mod impls;
pub mod scope;
pub mod stdlib;
pub mod symbol;
pub mod tests;
pub mod ty;
type RF<T> = Rc<RefCell<T>>;
use crate::ast::prelude::{
    Call, Expression, ExpressionWithSemi, Function, Identifier, Item, LetStatement, SourceFile,
    Statement,
};

use self::{
    scope::{Scope, ScopeArena},
    symbol::{InnerSymbol, Symbol},
    ty::{Type, TypeArena, TypeData, TypeRef},
};

#[derive(Debug)]
pub struct Typechecker<'a> {
    pub source_file: &'a mut SourceFile<'a>,
    pub ty_arena: TypeArena<'a>,
    pub scopes: ScopeArena<'a>,
}

impl<'a> Typechecker<'a> {
    pub fn check(&'a mut self) -> Result<(), &'static str> {
        let u_self = UnsafeCell::new(self);
        // lol
        let sself = unsafe { &mut *u_self.get() };
        let scope = sself.scopes.push();
        unsafe { &mut (*u_self.get()) }.load_stdlib();

        for item in &mut unsafe { &mut *u_self.get() }.source_file.ast {
            let sself = unsafe { &mut *u_self.get() };
            item.check(sself, &scope);
        }

        let sself = unsafe { &mut *u_self.get() };
        Ok(())
    }
}

pub trait GetSymbol {
    fn get_symbol(&self) -> Option<&Symbol>;
}
