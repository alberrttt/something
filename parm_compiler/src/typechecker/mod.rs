use std::{cell::UnsafeCell, rc::Rc, slice::Windows};
pub mod scope;
pub mod symbol;
pub mod ty;
use crate::ast::prelude::{Expression, Function, Item, SourceFile, Statement};

use self::{
    scope::{MutScopeRef, Scope, ScopeArena},
    ty::{Type, TypeArena, TypeData},
};

#[derive(Debug)]
pub struct Typechecker<'a> {
    pub source_file: &'a mut SourceFile<'a>,
    pub ty_arena: TypeArena,
    pub scopes: ScopeArena<'a>,
}

impl<'a> Typechecker<'a> {
    pub fn check(&'a mut self) -> Result<(), &'static str> {
        let u_self = UnsafeCell::new(self);

        // lol
        let sself = unsafe { &mut *u_self.get() };
        let scope = sself.scopes.push();
        for item in &mut unsafe { &mut *u_self.get() }.source_file.ast {
            let sself = unsafe { &mut *u_self.get() };
            item.check(sself, scope.clone());
        }
        let sself = unsafe { &mut *u_self.get() };
        dbg!(&sself.scopes);
        Ok(())
    }
}

impl<'a> Item<'a> {
    fn check(&mut self, tc: &'a mut Typechecker<'a>, with: MutScopeRef<'a>) -> () {
        match self {
            Item::Function(func) => func.check(tc, with),
            _ => panic!(),
        }
    }
}
impl<'a> Expression<'a> {
    fn check(&mut self, tc: &mut Typechecker, with: ()) -> () {}
}

impl<'a> Function<'a> {
    fn check<'b: 'a>(&mut self, tc: &'b mut Typechecker<'a>, with: MutScopeRef<'b>) -> () {
        let mut scope = tc.scopes.insert(with.idx);
        let mut tc = UnsafeCell::new(tc);
        for param in self.params.collect_t() {
            let mut tc = unsafe { &mut *tc.get() };
            let ty = Type::ty_expr(&param.annotation.ty);
            let ty = ty.allocate(&mut tc.ty_arena);
            let symbol = symbol::Symbol {
                inner: Rc::new(symbol::InnerSymbol {
                    source_file: tc.source_file,
                    name: param.name.lexeme,
                    ty,
                }),
            };
            scope.vars.insert(param.name.lexeme, symbol);
        }

        let mut tc = unsafe { &mut *tc.get() };
        for stmt in &mut self.body.statements.inner {
            stmt.check(tc, ());
        }
    }
}

impl<'a> Statement<'a> {
    fn check(&mut self, tc: &mut Typechecker, with: ()) -> () {}
}
