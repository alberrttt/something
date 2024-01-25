use std::{cell::UnsafeCell, rc::Rc, slice::Windows};
pub mod scope;
pub mod symbol;
pub mod ty;
use crate::ast::prelude::{
    Expression, ExpressionWithSemi, Function, Identifier, Item, LetStatement, SourceFile, Statement,
};

use self::{
    scope::{MutScopeRef, Scope, ScopeArena},
    ty::{Type, TypeArena, TypeData, TypeRef},
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
    fn check<'b: 'a>(&'b mut self, tc: &'b mut Typechecker<'a>, with: MutScopeRef<'a>) -> () {
        match self {
            Item::Function(func) => func.check(tc, with),
            _ => panic!(),
        }
    }
}
impl<'a> Expression<'a> {
    fn check(&mut self, tc: &mut Typechecker<'a>, with: &MutScopeRef<'a>) -> () {}
    pub fn get_ty<'b: 'a>(
        &self,
        tc: &'b mut Typechecker<'a>,
        with: MutScopeRef<'a>,
    ) -> TypeRef<'_> {
        match self {
            Expression::Identifier(ident) => ident.get_ty(tc, with),
            Expression::Number(_) => Type {
                data: TypeData::Number,
            }
            .allocate(&mut tc.ty_arena),
            _ => todo!(),
        }
    }
}
impl<'a> Identifier<'a> {
    pub fn get_ty(&self, tc: &Typechecker, with: MutScopeRef<'a>) -> TypeRef<'_> {
        let scope = &*with;
        let symbol = scope.vars.get(self.lexeme).unwrap();
        symbol.inner.ty.clone()
    }
}

impl<'a> Function<'a> {
    fn check<'b: 'a>(&'b mut self, tc: &'b mut Typechecker<'a>, with: MutScopeRef<'b>) {
        let tc = UnsafeCell::new(tc);

        let mut scope = unsafe { &mut *tc.get() }.scopes.insert(with.idx);
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

        for stmt in &mut self.body.statements.inner {
            let mut tc = unsafe { &mut *tc.get() };

            stmt.check(tc, &scope);
        }
    }
}
impl<'a> LetStatement<'a> {
    fn check<'b: 'a>(&'b mut self, tc: &'b mut Typechecker<'a>, mut with: MutScopeRef<'a>) {
        let tc = UnsafeCell::new(tc);
        let init = self.initializer.as_ref().unwrap();
        let ty = init.expr.get_ty(unsafe { &mut *tc.get() }, with.clone());
        let name = self.ident;
        let symbol = symbol::Symbol {
            inner: Rc::new(symbol::InnerSymbol {
                source_file: unsafe { &**tc.get() }.source_file,
                name: name.lexeme,
                ty,
            }),
        };

        with.vars.insert(name.lexeme, symbol);
    }
}
impl<'a> Statement<'a> {
    fn check<'b: 'a>(&'b mut self, tc: &'b mut Typechecker<'a>, with: &MutScopeRef<'a>) -> () {
        match self {
            Statement::Expression(expr) => expr.check(tc, with),
            Statement::ExpressionWithSemi(ExpressionWithSemi { expression, semi }) => {
                expression.check(tc, with)
            }
            Statement::Let(let_stmt) => let_stmt.check(tc, with.clone()),
            x => panic!("{:#?}", x),
        }
    }
}
