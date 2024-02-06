use std::borrow::Borrow;

use crate::typechecker::*;
impl<'a> Item<'a> {
    pub(super) fn check<'b: 'a>(
        &'b mut self,
        tc: &'b mut Typechecker<'a>,
        with: &RF<Scope<'a>>,
    ) -> () {
        match self {
            Item::Function(func) => func.check(tc, with),
            _ => panic!(),
        }
    }
}
impl<'a> Call<'a> {
    pub(super) fn check<'b: 'a>(
        &mut self,
        tc: &'b mut Typechecker<'a>,
        with: &RF<Scope<'a>>,
    ) -> TypeRef<'b> {
        let tc = UnsafeCell::new(tc);
        let scope: Rc<RefCell<Scope<'_>>> = unsafe { &mut *tc.get() }.scopes.insert({
            let with: &Scope<'_> = &with.as_ref().borrow();
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
        let Expression::Identifier(ref mut callee) = sself.callee.as_mut() else {
            panic!()
        };

        let function_decl = s_tc
            .scopes
            .get_symbol(with.as_ref().borrow().idx, callee.lexeme);
        let function_decl = match function_decl {
            Some(function_decl) => function_decl,
            None => panic!("Function `{}` not found", callee.lexeme),
        };
        callee.symbol = Some(function_decl.clone());
        let function_decl = function_decl.borrow();
        let inner = &function_decl.inner.as_ref().borrow();
        match &inner.ty.data {
            TypeData::Function { params, ret } => {
                let mut index = 0;
                loop {
                    let arg = args.get_mut(index);
                    let param = match params.get(index) {
                        Some(param) => param,
                        None => {
                            if arg.is_some() {
                                panic!("Too many arguments")
                            } else {
                                break;
                            }
                        }
                    };

                    let arg = match arg {
                        Some(arg) => arg,
                        None => panic!("Not enough arguments"),
                    };
                    let tc = unsafe { &mut *tc.get() };
                    let ty = arg.check(tc, with);
                    let ty = &*ty;
                    if (ty).ne(param)
                        && !matches!(
                            (&ty.data, &param.data),
                            (TypeData::Any, _) | (_, TypeData::Any)
                        )
                    {
                        panic!("Type mismatch")
                    }

                    index += 1;
                }
                *ret.clone()
            }
            x => {
                dbg!(x);
                panic!()
            }
        }
    }
}
impl<'a> Expression<'a> {
    pub(super) fn check<'b: 'a>(
        &mut self,
        tc: &'b mut Typechecker<'a>,
        with: &RF<Scope<'a>>,
    ) -> TypeRef<'b> {
        match self {
            Expression::Identifier(ident) => {
                let scope = &*with;
                let binding = scope.borrow_mut();
                let symbol = binding.vars.get(ident.lexeme).unwrap();
                ident.symbol = Some(symbol.clone());
                dbg!(&symbol);
                let inner = symbol.inner.as_ref().borrow();
                inner.ty.clone()
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
            Expression::StringLit(_) => Type {
                data: TypeData::String,
            }
            .allocate(&mut tc.ty_arena),
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
    pub(super) fn check<'b: 'a>(&'b mut self, tc: &'b mut Typechecker<'a>, with: &RF<Scope<'b>>) {
        let tc = UnsafeCell::new(tc);

        let mut scope = unsafe { &mut *tc.get() }.scopes.insert({
            let mut with = with.as_ref().borrow();
            with.idx
        });
        let mut params = Vec::new();
        for param in self.params.collect_t() {
            let mut tc = unsafe { &mut *tc.get() };
            let ty = Type::ty_expr(&param.annotation.ty);
            let ty = ty.allocate(&mut tc.ty_arena);
            params.push(ty.clone());
            let symbol = symbol::Symbol {
                inner: Rc::new(RefCell::new(symbol::InnerSymbol {
                    source_file: tc.source_file,
                    name: param.name.lexeme,
                    ty,
                })),
            };
            scope.borrow_mut().vars.insert(param.name.lexeme, symbol);
        }
        let mut s_tc = unsafe { &mut *tc.get() };
        let function_symbol = Symbol {
            inner: Rc::new(RefCell::new(symbol::InnerSymbol {
                source_file: s_tc.source_file,
                name: self.name.lexeme,
                ty: Type {
                    data: TypeData::Function {
                        params,
                        ret: Box::new(
                            Type {
                                data: TypeData::None,
                            }
                            .allocate(&mut unsafe { &mut *tc.get() }.ty_arena),
                        ),
                    },
                }
                .allocate(&mut unsafe { &mut *tc.get() }.ty_arena),
            })),
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
    pub(super) fn check<'b: 'a>(
        &'b mut self,
        tc: &'b mut Typechecker<'a>,
        mut with: &RF<Scope<'a>>,
    ) {
        let tc = UnsafeCell::new(tc);
        let init = self.initializer.as_mut().unwrap();
        let ty = init.expr.check(unsafe { *tc.get() }, with);
        let name = &self.ident.lexeme;
        let symbol = symbol::Symbol {
            inner: Rc::new(RefCell::new(symbol::InnerSymbol {
                source_file: unsafe { &**tc.get() }.source_file,
                name,
                ty,
            })),
        };
        self.ident.symbol = Some(symbol.clone());
        with.borrow_mut().vars.insert(name, symbol);
    }
}
impl<'a> Statement<'a> {
    pub(super) fn check<'b: 'a>(
        &'b mut self,
        tc: &'b mut Typechecker<'a>,
        with: &RF<Scope<'a>>,
    ) -> () {
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
