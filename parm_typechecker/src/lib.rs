#![feature(decl_macro)]
pub mod prelude;

use ast::*;
use prelude::*;
pub mod error;
pub mod symbol;
pub mod traits;
pub mod types;
use std::{
    backtrace::Backtrace,
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    error::Error,
    f32::consts::E,
    rc::Rc,
};

use error::{display_diagnostic, ErrorKind, InvalidOperand, Mismatch, TypeError, UndefinedSymbol};

use symbol::{Symbol, SymbolDeclaration};
use types::{FnSig, Number, String, Type};

use crate::error::Incompatible;

#[derive(Debug, Default)]
pub struct Scope<'a, 'b> {
    pub variables: HashMap<&'b str, Rc<RefCell<Symbol<'a, 'b>>>>,
    pub should_eval_to: Option<Rc<Type>>,
    pub evals_to: Option<Rc<Type>>,
    pub scopes: Vec<Scope<'a, 'b>>,
    pub current_sub_scope: usize,
}

impl<'a, 'b: 'a> Scope<'a, 'b> {
    pub fn get(&self, name: impl Into<&'a str>) -> Option<Rc<RefCell<Symbol<'a, 'b>>>> {
        self.variables.get(name.into()).cloned()
    }

    pub fn insert(&mut self, name: &'b str, symbol: Rc<RefCell<Symbol<'a, 'b>>>) {
        self.variables.insert(name, symbol);
    }
}

#[derive(Debug)]
pub struct TypeChecker<'a, 'b: 'a> {
    pub scope: RefCell<Scope<'a, 'b>>,
    pub source_file: Rc<SourceFile<'a>>,
}
/// the method to typecheck an ast node is just its name.
impl<'b, 'a: 'b> TypeChecker<'a, 'b> {
    pub fn current_scope<'c: 'b>(&'c self) -> &'c Scope<'a, 'b> {
        &*self.mut_current_scope()
    }
    /// lol 😂
    #[allow(clippy::mut_from_ref)]
    pub fn mut_current_scope(&self) -> &mut Scope<'a, 'b> {
        let mut scope = unsafe { &mut *self.scope.as_ptr() };
        loop {
            println!("hello");
            if !scope.scopes.is_empty() {
                scope = &mut scope.scopes[scope.current_sub_scope]
            } else {
                break;
            }
        }

        scope
    }
    pub fn load_stdlib(&self) {
        let scope = self.mut_current_scope();
        scope.insert(
            "println",
            Rc::new(RefCell::new(Symbol {
                declaration: Some(SymbolDeclaration::None),
                ty: Rc::new(Type::FnSig(FnSig {
                    params: vec![],
                    return_type: Rc::new(Type::Void),
                })),
            })),
        )
    }
    pub fn typecheck(&'b self) {
        self.scope.borrow_mut().scopes.push(Scope::default());
        self.load_stdlib();
        self.hoist_functions(
            &self.source_file.ast,
            self.scope.borrow_mut().scopes.last_mut().unwrap(),
        );
        for (index, item) in self.source_file.ast.iter().enumerate() {
            self.item(item);
        }
    }
    pub fn hoist_functions(&'b self, items: &'b [Item<'a>], scope: &mut Scope<'a, 'b>) {
        for item in items {
            if let Item::Function(function) = item {
                let params = function.params.inner.inner.iter().map(|(param, _)| {
                    let ty = Type::try_from(&param.annotation.ty).unwrap();
                    (param.name.lexeme, ty)
                });
                let return_ty = match function.ret_type.as_ref() {
                    Some(ret) => Type::try_from(&ret.ret_type).unwrap(),
                    None => Type::Void,
                };
                let fn_sig = FnSig {
                    params: params.map(|(_, ty)| ty).collect(),
                    return_type: Rc::new(return_ty),
                };
                let symbol = Rc::new(RefCell::new(Symbol {
                    declaration: Some(SymbolDeclaration::Function(function)),
                    ty: Rc::new(Type::FnSig(fn_sig)),
                }));
                scope.insert(function.name.lexeme, symbol);
            }
        }
    }
    pub fn item(&'b self, item: &'b Item<'a>) {
        match item {
            Item::Use(use_stmt) => self.use_stmt(use_stmt),
            Item::Function(function) => self.function(function),
            Item::Struct(struct_) => self.struct_(struct_),

            item => todo!("{:?}", item),
        }
    }

    pub fn get_symbol(&'b self, ident: &str) -> Option<Rc<RefCell<Symbol<'a, 'b>>>> {
        let mut scope = &*self.scope.borrow();
        dbg!(&scope);
        let mut possible: Option<Rc<RefCell<Symbol<'_, '_>>>> = None;
        loop {
            println!("fix thiss");
            match scope.variables.get(ident) {
                Some(ty) => {
                    possible = Some(ty.clone());
                }
                None => {
                    if !scope.scopes.is_empty() {
                        scope = &scope.scopes[scope.current_sub_scope]
                    } else {
                        return possible;
                    }
                }
            }
        }
        possible
    }

    pub fn expression(&'b self, expr: &'a Expression<'a>) -> TypeResult<'b, Rc<Type>> {
        match expr {
            Expression::BinaryExpression(binary) => self.binary_expression(binary),
            Expression::Group(group) => self.expression(&group.paren.inner),
            Expression::Number(number) => {
                let value = number.value;
                let is_integer = value.fract() == 0.0;
                Ok(Rc::new(Type::Number(Number {})))
            }
            Expression::Identifier(identifier) => {
                let symbol = match self.get_symbol(identifier.lexeme) {
                    Some(symbol) => symbol,
                    None => {
                        let error = TypeError::new(
                            ErrorKind::UndefinedSymbol(UndefinedSymbol {
                                symbol: identifier.lexeme,
                                location: identifier.span(),
                            }),
                            self.source_file.as_ref(),
                        );
                        return Err(error);
                    }
                };
                let symbol = (*symbol).borrow();
                Ok(symbol.ty.clone())
            }
            Expression::Call(call) => {
                let callee = call.callee.as_ref();
                let callee = self.expression(callee)?;
                let fn_sig = match callee.as_ref() {
                    Type::FnSig(fn_sig) => fn_sig,
                    _ => todo!(),
                };
                let params = fn_sig.params.iter().cloned();
                let args = call.arguments.collect_t();
                for (param, arg) in params.zip(args) {
                    let arg_ty = self.expression(arg)?;
                    if arg_ty.as_ref() != &param {
                        let error = TypeError::new(
                            ErrorKind::Mismatch(Mismatch {
                                got: arg_ty,
                                expected: Rc::new(param),
                                location: arg.span(),
                            }),
                            self.source_file.as_ref(),
                        );
                        return Err(error);
                    }
                }
                Ok(fn_sig.return_type.clone())
            }
            Expression::StringLit(string) => Ok(Rc::new(Type::String(String {}))),
            Expression::If(if_expression) => {
                let condition = self.expression(&if_expression.condition)?;
                if &*condition != &Type::Boolean {
                    let err = TypeError::new(
                        ErrorKind::Mismatch(Mismatch {
                            got: condition,
                            expected: Rc::new(Type::Boolean),
                            location: if_expression.condition.span(),
                        }),
                        self.source_file.as_ref(),
                    );

                    return Err(err);
                }
                let body = &if_expression.body;

                Ok(self.block(body).unwrap().clone())
            }
            _ => todo!(),
        }
    }

    // this expects that a scope has already been created for the new block
    pub fn block_manual_scope(&'b self, block: &'b Block<'a>) -> TypeResult<'_, Rc<Type>> {
        let mut iter = block.statements.iter().enumerate();
        for (idx, statement) in iter {
            match self.statement(statement) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
            let is_last = idx == block.statements.len() - 1;
            if is_last {
                let scope = self.current_scope();
                let evals_to = scope.evals_to.clone().unwrap_or(Rc::new(Type::Void));
                return Ok(evals_to);
            }
        }
        panic!()
    }

    /// this creates a scope for the new block

    pub fn block(&'b self, block: &'b Block<'a>) -> TypeResult<'_, Rc<Type>> {
        let current_scope = self.mut_current_scope();
        let prev_scope = current_scope.current_sub_scope;
        current_scope.current_sub_scope = current_scope.scopes.len();
        current_scope.scopes.push(Scope::default());
        return {
            let tmp = self.block_manual_scope(block);
            current_scope.current_sub_scope = prev_scope;
            tmp
        };
    }
    pub fn binary_expression(
        &'b self,
        binary: &'a BinaryExpression<'a>,
    ) -> TypeResult<'b, Rc<Type>> {
        let lhs_ty = self.expression(&binary.left)?;
        let rhs_ty = self.expression(&binary.right)?;
        if binary.operator.is_boolean_logic_operator() {
            if let (Type::Boolean, Type::Boolean) = (&*lhs_ty, &*rhs_ty) {
            } else {
                let err = TypeError::new(
                    ErrorKind::InvalidOperand(InvalidOperand {
                        operand: binary.operator.lexeme(),
                        location: binary.span(),
                        type1: lhs_ty.clone(),
                        type2: Some(rhs_ty.clone()),
                    }),
                    &self.source_file,
                );
                return Err(err);
            }
        }
        if binary.operator.is_comparison() {
            if let (Type::Number(_), Type::Number(_)) = (&*lhs_ty, &*rhs_ty) {
                return Ok(Rc::new(Type::Boolean));
            } else {
                let err = TypeError::new(
                    ErrorKind::InvalidOperand(InvalidOperand {
                        operand: binary.operator.lexeme(),
                        location: binary.span(),
                        type1: lhs_ty,
                        type2: Some(rhs_ty),
                    }),
                    &self.source_file,
                );
                return Err(err);
            }
        }
        if lhs_ty.as_ref() == rhs_ty.as_ref() {
            return Ok(lhs_ty);
        }
        if binary.operator.is_equality() {
            let warning = display_diagnostic(
                &self.source_file,
                binary.span(),
                "Under equality, this expression will always be false",
            );
            println!("{}", warning.unwrap());
            return Ok(Rc::new(Type::Boolean));
        }
        let err = TypeError::new(
            ErrorKind::InvalidOperand(InvalidOperand {
                operand: binary.operator.lexeme(),
                location: binary.span(),
                type1: lhs_ty,
                type2: Some(rhs_ty),
            }),
            &self.source_file,
        );
        Err(err)
    }
    pub fn statement(&'b self, statement: &'a Statement<'a>) -> TypeResult<'b, ()> {
        match statement {
            Statement::Expression(expression) => {
                let ty = self.expression(expression)?;
            }
            Statement::ExpressionWithSemi(ExpressionWithSemi { expression, semi }) => {
                self.expression(expression)?;
            }
            Statement::Item(item) => {
                self.item(item);
            }
            Statement::Let(let_) => {
                self.variable(let_)?;
            }
            Statement::Return(return_) => {
                self.return_stmt(return_)?;
            }
        };
        Ok(())
    }
    pub fn return_stmt(&'b self, stmt: &'a ReturnStatement<'a>) -> TypeResult<'b, ()> {
        let ty = self.expression(&stmt.expr)?;
        let scope = self.mut_current_scope();
        let should_eval_to = scope.should_eval_to.as_ref().unwrap();
        if *ty != **should_eval_to {
            let err = TypeError::new(
                ErrorKind::Incompatible(Incompatible {
                    type1: ty,
                    type2: should_eval_to.clone(),
                    location: stmt.span(),
                }),
                self.source_file.as_ref(),
            );
            return Err(err);
        }
        scope.evals_to = Some(ty);
        Ok(())
    }
    pub fn variable(&'b self, variable: &'a LetStatement<'a>) -> TypeResult<'b, ()> {
        let ty = self.expression(&variable.initializer.as_ref().unwrap().expr)?;
        let scope = self.mut_current_scope();

        scope.variables.insert(
            variable.ident.lexeme,
            Rc::new(RefCell::new(Symbol {
                declaration: Some(symbol::SymbolDeclaration::Variable(&variable)),
                ty,
            })),
        );
        Ok(())
    }
    pub fn struct_(&self, struct_: &Struct) {}
    pub fn use_stmt(&self, use_stmt: &UseStatement<'a>) {}
    pub fn param(&self, param: &'a Param<'a>) {
        let ty = &param.annotation.ty;
        let ty = Type::try_from(ty).unwrap();
        let scope = self.mut_current_scope();
        scope.variables.insert(
            param.name.lexeme,
            Rc::new(RefCell::new(Symbol {
                declaration: Some(symbol::SymbolDeclaration::Param(param)),
                ty: Rc::new(ty),
            })),
        );
    }
    pub fn function(&'b self, function: &'b Function<'a>) {
        let mut scope = self.mut_current_scope();
        let tmp = scope.current_sub_scope;
        scope.current_sub_scope = scope.scopes.len();
        scope.scopes.push(Scope {
            variables: HashMap::default(),
            should_eval_to: Some(
                match function.ret_type.as_ref() {
                    Some(ret) => Type::try_from(&ret.ret_type).unwrap(),
                    None => Type::Void,
                }
                .into(),
            ),
            evals_to: None,
            scopes: Vec::new(),
            current_sub_scope: 0,
        });
        for (param, _) in function.params.inner.inner.iter() {
            self.param(param);
        }
        if let Some(last_param) = function.params.last.as_ref() {
            self.param(last_param);
        }
        self.block_manual_scope(&function.body);
        scope.current_sub_scope = tmp;
    }
}
