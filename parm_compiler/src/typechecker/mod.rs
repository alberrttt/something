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
        {
            scope.borrow_mut().vars.insert(
                "println",
                Symbol {
                    inner: Rc::new(InnerSymbol {
                        source_file: sself.source_file,
                        name: "println",
                        ty: Type {
                            data: TypeData::Function {
                                params: Vec::new(),
                                ret: Box::new(
                                    Type {
                                        data: TypeData::None,
                                    }
                                    .allocate(&mut sself.ty_arena),
                                ),
                            },
                        }
                        .allocate(&mut sself.ty_arena),
                    }),
                },
            );
        }
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
    fn check<'b: 'a>(&mut self, tc: &'b mut Typechecker<'a>, with: &RF<Scope<'a>>) -> TypeRef<'b> {
        let tc = UnsafeCell::new(tc);
        let scope = unsafe { &mut *tc.get() }.scopes.insert({
            let with = with.borrow();
            with.idx
        });

        let sself = UnsafeCell::new(self);
        let mut args = unsafe { &mut *sself.get() }
            .arguments
            .inner
            .inner
            .iter_mut()
            .map(|f| &mut f.0)
            .collect::<Vec<_>>();
        let sself = unsafe { &mut *sself.get() };
        if let Some(arg) = sself.arguments.last.as_mut() {
            args.push(arg.as_mut());
        }
        let s_tc = unsafe { &mut *tc.get() };
        let Expression::Identifier(callee) = sself.callee.as_ref() else {
            panic!()
        };

        let function_decl = s_tc
            .scopes
            .get_variable(with.borrow().idx, callee.lexeme)
            .unwrap();
        for arg in args.iter_mut() {
            let tc = unsafe { &mut *tc.get() };
            let arg_ty = arg.check(tc, &scope);
        }
        match &function_decl.data {
            TypeData::Function { params, ret } => {
                todo!();
                return *ret.clone();
            }
            _ => panic!(),
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
            Expression::Call(call) => call.check(tc, with),

            _ => todo!("{:#?}", self),
        }
    }
}
impl<'a> Identifier<'a> {
    pub fn get_symbol(&self) -> Option<&symbol::Symbol<'a>> {
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
        let mut params = Vec::new();
        for param in self.params.collect_t() {
            let mut tc = unsafe { &mut *tc.get() };
            let ty = Type::ty_expr(&param.annotation.ty);
            let ty = ty.allocate(&mut tc.ty_arena);
            params.push(ty.clone());
            let symbol = symbol::Symbol {
                inner: Rc::new(symbol::InnerSymbol {
                    source_file: tc.source_file,
                    name: param.name.lexeme,
                    ty,
                }),
            };
            scope.borrow_mut().vars.insert(param.name.lexeme, symbol);
        }
        let mut s_tc = unsafe { &mut *tc.get() };
        let function_symbol = Symbol {
            inner: Rc::new(symbol::InnerSymbol {
                source_file: s_tc.source_file,
                name: self.name.lexeme,
                ty: Type {
                    data: TypeData::Function {
                        params,
                        ret: Box::new(
                            Type {
                                data: TypeData::None,
                            }
                            .allocate(&mut s_tc.ty_arena),
                        ),
                    },
                }
                .allocate(&mut s_tc.ty_arena),
            }),
        };
        with.borrow_mut()
            .vars
            .insert(self.name.lexeme, function_symbol.clone());
        self.name.symbol = Some(function_symbol.clone());
        for stmt in &mut self.body.statements.inner {
            let tc = unsafe { &mut *tc.get() };

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
