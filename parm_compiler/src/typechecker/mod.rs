use std::cell::UnsafeCell;
mod scope;

use crate::ast::prelude::{Expression, Function, Item, SourceFile, Statement};

use self::scope::ScopeArena;

#[derive(Debug, PartialEq)]
pub struct Typechecker<'a, 'file> {
    pub source_file: &'file mut SourceFile<'a>,
    pub scopes: ScopeArena,
}

impl<'a, 'b> Typechecker<'a, 'b> {
    pub fn check(&mut self) -> Result<(), &'static str> {
        let u_self = UnsafeCell::new(self);

        // lol
        for item in &mut unsafe { &mut *u_self.get() }.source_file.ast {
            item.check(unsafe { *u_self.get() });
        }
        Ok(())
    }
}

trait Check {
    fn check(&mut self, tc: &mut Typechecker) -> ();
}
impl<'a> Check for Item<'a> {
    fn check(&mut self, tc: &mut Typechecker) -> () {
        match self {
            Item::Function(func) => func.check(tc),
            _ => panic!(),
        }
    }
}
impl<'a> Check for Expression<'a> {
    fn check(&mut self, tc: &mut Typechecker) -> () {}
}

impl<'a> Check for Function<'a> {
    fn check(&mut self, tc: &mut Typechecker) -> () {
        for stmt in &mut self.body.statements.inner {
            stmt.check(tc);
        }
    }
}

impl<'a> Check for Statement<'a> {
    fn check(&mut self, tc: &mut Typechecker) -> () {}
}
