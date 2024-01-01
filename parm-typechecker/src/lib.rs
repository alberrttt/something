#![feature(decl_macro)]
use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

use error::{display_diagnostic, ErrorKind, InvalidOperand, Mismatch, TypeError};
use parm_ast::{
    parser::nodes::{
        expression::call,
        item::ReturnStatement,
        statement::{self, ExpressionWithSemi},
    },
    prelude::*,
};
use symbol::Symbol;
use types::{FnSig, Number, String, Type};

use crate::error::Incompatible;
mod error;
mod symbol;
mod traits;
mod types;
#[derive(Debug, Default)]
pub struct Scope<'a> {
    pub variables: HashMap<&'a str, Rc<RefCell<Symbol<'a>>>>,
    pub should_eval_to: Option<Type>,
}

impl<'a> Scope<'a> {
    /// O(n)
    pub fn get(&self, name: impl Into<&'a str>) -> Option<Rc<RefCell<Symbol<'a>>>> {
        self.variables.get(name.into()).cloned()
    }
}

#[derive(Debug)]
pub struct TypeCheckedSourceFile<'a> {
    pub source_file: Rc<SourceFile<'a>>,
    pub typechecker: TypeChecker<'a>,
}

impl<'a> TypeCheckedSourceFile<'a> {
    pub fn new(source_file: SourceFile<'a>) -> Self {
        let source_file = Rc::new(source_file);
        Self {
            source_file: source_file.clone(),
            typechecker: TypeChecker {
                scopes: vec![],
                source_file: source_file.clone(),
            },
        }
    }
    pub fn typecheck(&'a mut self) {
        let mut scope = Scope::default();
        for item in &self.source_file.ast {
            if let Item::Function(function) = item {
                let return_type = match function.ret_type.as_ref() {
                    Some(ret) => Type::try_from(&ret.ret_type).unwrap(),
                    None => Type::Void,
                };
                let mut params: Vec<Type> = Vec::new();

                for param in function.params.inner.collect_t() {
                    let ty = &param.annotation.ty;
                    let ty = <Type as TryFrom<&TypeExpression<'_>>>::try_from(ty).unwrap();
                    params.push(ty);
                }
                scope.variables.insert(
                    function.name.lexeme,
                    Rc::new(RefCell::new(Symbol {
                        declaration: Some(symbol::SymbolDeclaration::Function(&function)),
                        ty: Type::FnSig(FnSig {
                            params,
                            return_type: Box::new(return_type),
                        }),
                    })),
                );
            }
        }
        self.typechecker.scopes.push(scope);

        for (index, item) in self.source_file.ast.iter().enumerate() {
            self.typechecker.item(&item);
        }
    }
}

#[derive(Debug)]
pub struct TypeChecker<'a> {
    pub scopes: Vec<Scope<'a>>,
    pub source_file: Rc<SourceFile<'a>>,
}
/// the method to typecheck an ast node is just its name.
impl<'b, 'a: 'b> TypeChecker<'a> {
    /// typechecks an item

    pub fn item(&mut self, item: &'a Item<'a>) {
        match item {
            Item::Use(use_stmt) => self.use_stmt(use_stmt),
            Item::Function(function) => self.function(function),
            Item::Struct(struct_) => self.struct_(struct_),

            item => todo!("{:?}", item),
        }
    }
    pub fn scope(&'b self) -> &'b Scope<'a> {
        self.scopes.last().as_ref().unwrap()
    }
    pub fn get_symbol(&self, ident: &str) -> Option<Rc<RefCell<Symbol<'a>>>> {
        let mut scope = self.scope();
        let mut idx: usize = self.scopes.len();
        loop {
            match scope.variables.get(ident) {
                Some(ty) => return Some(ty.clone()),
                None => {
                    if idx == 0 {
                        return None;
                    }
                    idx -= 1;
                    scope = match self.scopes.get(idx) {
                        Some(scope) => scope,
                        None => {
                            return None;
                        }
                    };
                }
            }
        }
    }

    pub fn expression(&mut self, expr: &'b Expression<'a>) -> Type {
        match expr {
            Expression::BinaryExpression(binary) => self.binary_expression(binary),
            Expression::Group(group) => self.expression(&group.paren.inner),
            Expression::Number(number) => {
                let value = number.value;
                let is_integer = value.fract() == 0.0;
                Type::Number(Number {})
            }
            Expression::Identifier(identifier) => {
                let symbol = match self.get_symbol(identifier.lexeme) {
                    Some(symbol) => symbol,
                    None => return Type::Void,
                };
                let symbol = symbol.as_ref().borrow();
                symbol.ty.clone()
            }
            Expression::Call(call) => {
                let callee = call.callee.as_ref();
                let callee: Type = self.expression(callee);
                let fn_sig = match callee {
                    Type::FnSig(fn_sig) => fn_sig,
                    _ => todo!(),
                };
                let params = fn_sig.params.iter().cloned();
                let args = call.arguments.collect_t();
                for (param, arg) in params.zip(args) {
                    let arg_ty = self.expression(arg);
                    if arg_ty != param {
                        let error = TypeError::new(
                            ErrorKind::Mismatch(Mismatch {
                                got: &arg_ty,
                                expected: &param,
                                location: arg.span(),
                            }),
                            self.source_file.as_ref(),
                        );
                        eprintln!("{}", error);
                    }
                }
                *fn_sig.return_type.clone()
            }
            Expression::StringLit(string) => Type::String(String {}),
            _ => todo!(),
        }
    }
    pub fn binary_expression(&mut self, binary: &'b BinaryExpression<'a>) -> Type {
        let lhs_ty = self.expression(&binary.left);
        let rhs_ty = self.expression(&binary.right);
        if binary.operator.is_boolean_operator() {
            if let (Type::Boolean(_), Type::Boolean(_)) = (&lhs_ty, &rhs_ty) {
            } else {
                let err = TypeError::new(
                    ErrorKind::InvalidOperand(InvalidOperand {
                        operand: binary.operator.lexeme(),
                        location: binary.span(),
                        type1: &lhs_ty,
                        type2: Some(&rhs_ty),
                    }),
                    &self.source_file,
                );
                eprintln!("{}", err);
            }
        }
        if lhs_ty == rhs_ty {
            return lhs_ty;
        }
        if binary.operator.is_equality() {
            let warning = display_diagnostic(
                &self.source_file,
                binary.span(),
                "Under equality, this expression will always be false",
            );
            println!("{}", warning.unwrap());
        }
        let err = TypeError::new(
            ErrorKind::InvalidOperand(InvalidOperand {
                operand: binary.operator.lexeme(),
                location: binary.span(),
                type1: &lhs_ty,
                type2: Some(&rhs_ty),
            }),
            &self.source_file,
        );
        eprintln!("{}", err);
        lhs_ty
    }
    pub fn statement(&mut self, statement: &'a Statement<'a>) {
        match statement {
            Statement::Expression(expression) => {
                self.expression(expression);
            }
            Statement::ExpressionWithSemi(ExpressionWithSemi { expression, semi }) => {
                self.expression(expression);
            }
            Statement::Item(item) => {
                self.item(item);
            }
            Statement::Let(let_) => {
                self.variable(let_);
            }
            Statement::Return(return_) => {
                self.return_stmt(return_);
            }
        };
    }
    pub fn return_stmt(&mut self, stmt: &'b ReturnStatement<'a>) {
        let ty = self.expression(&stmt.expr);
        let scope = self.scope();
        let should_eval_to = scope.should_eval_to.as_ref().unwrap();
        if ty != *should_eval_to {
            let err = TypeError::new(
                ErrorKind::Incompatible(Incompatible {
                    type1: &ty,
                    type2: should_eval_to,
                    location: stmt.span(),
                }),
                self.source_file.as_ref(),
            );
            eprintln!("{}", err);
        }
    }
    pub fn variable(&mut self, variable: &'a LetStmt<'a>) {
        let ty = self.expression(&variable.initializer.as_ref().unwrap().expr);
        let scope = self.scopes.last_mut().unwrap();

        scope.variables.insert(
            variable.ident.lexeme,
            Rc::new(RefCell::new(Symbol {
                declaration: Some(symbol::SymbolDeclaration::Variable(&variable)),
                ty,
            })),
        );
    }
    pub fn struct_(&mut self, struct_: &Struct) {}
    pub fn use_stmt(&mut self, use_stmt: &UseStatement<'a>) {}
    pub fn param(&mut self, param: &'a Param<'a>) {
        let ty = &param.annotation.ty;
        let ty = Type::try_from(ty).unwrap();

        let scope = self.scopes.last_mut().unwrap();
        scope.variables.insert(
            param.name.lexeme,
            Rc::new(RefCell::new(Symbol {
                declaration: Some(symbol::SymbolDeclaration::Param(&param)),
                ty,
            })),
        );
    }
    pub fn function(&mut self, function: &'a Function<'a>) {
        self.scopes.push(Scope {
            variables: HashMap::default(),
            should_eval_to: Some(match function.ret_type.as_ref() {
                Some(ret) => Type::try_from(&ret.ret_type).unwrap(),
                None => Type::Void,
            }),
        });
        for (param, _) in function.params.inner.inner.iter() {
            self.param(param);
        }
        if let Some(last_param) = function.params.last.as_ref() {
            self.param(last_param);
        }
        for statement in function.body.iter() {
            self.statement(statement);
        }
        self.scopes.pop();
    }
}
