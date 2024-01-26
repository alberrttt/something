use std::{
    cell::{RefCell, UnsafeCell},
    rc::Rc,
    slice::Windows,
};
pub mod scope;
pub mod symbol;
pub mod ty;
type RF<T> = Rc<RefCell<T>>;
use crate::ast::prelude::{
    Call, Expression, ExpressionWithSemi, Function, Identifier, Item, LetStatement, SourceFile,
    Statement,
};

use self::{
    scope::{Scope, ScopeArena},
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
            item.check(sself, &scope);
        }
        let sself = unsafe { &mut *u_self.get() };
        Ok(())
    }
}

impl<'a> Item<'a> {
    fn check<'b: 'a>(&'b mut self, tc: &'b mut Typechecker<'a>, with: &RF<Scope<'a>>) -> () {
        match self {
            Item::Function(func) => func.check(tc, with),
            _ => panic!(),
        }
    }
}
impl<'a> Call<'a> {
    fn check<'b: 'a>(&'b mut self, tc: &'b mut Typechecker<'a>, with: &RF<Scope<'a>>) -> () {
        let tc = UnsafeCell::new(tc);
        let mut scope = unsafe { &mut *tc.get() }.scopes.insert({
            let mut with = with.borrow();
            with.idx
        });
        let mut args = self
            .arguments
            .inner
            .inner
            .iter_mut()
            .map(|f| &mut f.0)
            .collect::<Vec<_>>();
        if let Some(mut arg) = &self.arguments.last {
            args.push(arg.as_mut());
        }
        for arg in args {
            let mut tc = unsafe { &mut *tc.get() };
            arg.check(tc, &scope);
        }
    }
}
impl<'a> Expression<'a> {
    fn check<'b: 'a>(&mut self, tc: &'b mut Typechecker<'a>, with: &RF<Scope<'a>>) -> TypeRef<'b> {
        match self {
            Expression::Identifier(ident) => {
                let scope = &*with;
                let binding = scope.borrow_mut();
                let symbol = binding.vars.get(ident.lexeme).unwrap();
                ident.symbol = Some(symbol.clone());
                dbg!(&symbol);
                symbol.inner.ty.clone()
            }
            Expression::Number(_) => Type {
                data: TypeData::Number,
            }
            .allocate(&mut tc.ty_arena),
            Expression::BinaryExpression(_) => Type {
                data: TypeData::Number,
            }
            .allocate(&mut tc.ty_arena),
            Expression::Call(call) => Type {
                data: TypeData::Number,
            }
            .allocate(&mut tc.ty_arena),

            _ => todo!("{:#?}", self),
        }
    }
}
impl<'a> Identifier<'a> {
    pub fn get_symbol(&self) -> Option<&symbol::Symbol<'_>> {
        return self.symbol.as_ref();
    }
}

impl<'a> Function<'a> {
    fn check<'b: 'a>(&'b mut self, tc: &'b mut Typechecker<'a>, with: &RF<Scope<'b>>) {
        let tc = UnsafeCell::new(tc);

        let mut scope = unsafe { &mut *tc.get() }.scopes.insert({
            let mut with = with.borrow();
            with.idx
        });

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
            scope.borrow_mut().vars.insert(param.name.lexeme, symbol);
        }

        for stmt in &mut self.body.statements.inner {
            let mut tc = unsafe { &mut *tc.get() };

            stmt.check(tc, &scope);
        }
    }
}
impl<'a> LetStatement<'a> {
    fn check<'b: 'a>(&'b mut self, tc: &'b mut Typechecker<'a>, mut with: &RF<Scope<'a>>) {
        let tc = UnsafeCell::new(tc);
        let init = self.initializer.as_mut().unwrap();
        let ty = init.expr.check(unsafe { &mut *tc.get() }, with);
        let name = &self.ident;
        let symbol = symbol::Symbol {
            inner: Rc::new(symbol::InnerSymbol {
                source_file: unsafe { &**tc.get() }.source_file,
                name: name.lexeme,
                ty,
            }),
        };

        with.borrow_mut().vars.insert(name.lexeme, symbol);
    }
}
impl<'a> Statement<'a> {
    fn check<'b: 'a>(&'b mut self, tc: &'b mut Typechecker<'a>, with: &RF<Scope<'a>>) -> () {
        match self {
            Statement::Expression(expr) => {
                expr.check(tc, with);
            }
            Statement::ExpressionWithSemi(ExpressionWithSemi { expression, semi }) => {
                expression.check(tc, with);
            }
            Statement::Let(let_stmt) => let_stmt.check(tc, with),
            x => panic!("{:#?}", x),
        }
    }
}
