#![feature(decl_macro)]
pub mod prelude;

use ast::*;
use prelude::*;
pub mod error;
pub mod symbol;
pub mod traits;
pub mod types;
use std::{
    borrow::BorrowMut, cell::RefCell, collections::HashMap, error::Error, f32::consts::E, rc::Rc,
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
    pub expression_types: Vec<Rc<Type>>,
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
pub struct TypeCheckedSourceFile<'a, 'b: 'a> {
    pub source_file: Rc<SourceFile<'a>>,
    pub typechecker: TypeChecker<'a, 'b>,
}

impl<'a, 'b: 'a> TypeCheckedSourceFile<'a, 'b> {
    pub fn new(source_file: SourceFile<'a>) -> Self {
        let source_file = Rc::new(source_file);
        Self {
            source_file: source_file.clone(),
            typechecker: TypeChecker {
                scopes: RefCell::new(vec![]),
                source_file: source_file.clone(),
            },
        }
    }
    pub fn typecheck(&'a mut self) {
        self.typechecker.typecheck();
    }
}

#[derive(Debug)]
pub struct TypeChecker<'a, 'b: 'a> {
    pub scopes: RefCell<Vec<Scope<'a, 'b>>>,
    pub source_file: Rc<SourceFile<'a>>,
}
/// the method to typecheck an ast node is just its name.
impl<'b, 'a: 'b> TypeChecker<'a, 'b> {
    pub fn load_stdlib(&self) {
        let mut scopes = self.scopes.borrow_mut();
        let scope = scopes.last_mut().unwrap();
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
    pub fn typecheck(&'b mut self) {
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
                        declaration: Some(symbol::SymbolDeclaration::Function(function)),
                        ty: Rc::new(Type::FnSig(FnSig {
                            params,
                            return_type: Rc::new(return_type),
                        })),
                    })),
                );
            }
        }
        self.scopes.borrow_mut().push(scope);
        self.load_stdlib();
        for (index, item) in self.source_file.ast.iter().enumerate() {
            self.item(&item);
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
    pub fn scope<'c>(&'c self) -> &'c Scope<'a, 'b> {
        unsafe { &*self.scopes.as_ptr() }.last().unwrap()
    }
    pub fn scopes<'c>(&'c self) -> &'c Vec<Scope<'a, 'b>> {
        unsafe { &*self.scopes.as_ptr() }
    }

    pub fn get_symbol(&'b self, ident: &str) -> Option<Rc<RefCell<Symbol<'a, 'b>>>> {
        let mut scope = self.scope();
        let mut idx: usize = self.scopes.borrow().len();
        loop {
            match scope.variables.get(ident) {
                Some(ty) => return Some(ty.clone()),
                None => {
                    if idx == 0 {
                        return None;
                    }
                    idx -= 1;
                    let scopes = self.scopes();
                    scope = match scopes.get(idx) {
                        Some(scope) => scope,
                        None => {
                            return None;
                        }
                    };
                }
            }
        }
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
                let symbol = symbol.borrow();
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

    /// this expects that a scope has already been created for the new block
    pub fn block(&'b self, block: &'b Block<'a>) -> TypeResult<'_, Rc<Type>> {
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
                let scope = self.scope();
                let evals_to = scope.evals_to.clone().unwrap_or(Rc::new(Type::Void));
                return Ok(evals_to);
            }
        }
        panic!()
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
                self.scopes
                    .borrow_mut()
                    .last_mut()
                    .unwrap()
                    .expression_types
                    .push(ty);
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
        let scope = self.scope();
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
        self.scopes.borrow_mut().last_mut().unwrap().evals_to = Some(ty);
        Ok(())
    }
    pub fn variable(&'b self, variable: &'a LetStatement<'a>) -> TypeResult<'b, ()> {
        let ty = self.expression(&variable.initializer.as_ref().unwrap().expr)?;
        let mut scopes = self.scopes.borrow_mut();
        let scope = scopes.last_mut().unwrap();

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
        let mut scopes = self.scopes.borrow_mut();
        let scope = scopes.last_mut().unwrap();
        scope.variables.insert(
            param.name.lexeme,
            Rc::new(RefCell::new(Symbol {
                declaration: Some(symbol::SymbolDeclaration::Param(&param)),
                ty: Rc::new(ty),
            })),
        );
    }
    pub fn function(&'b self, function: &'b Function<'a>) {
        self.scopes.borrow_mut().push(Scope {
            variables: HashMap::default(),
            should_eval_to: Some(
                match function.ret_type.as_ref() {
                    Some(ret) => Type::try_from(&ret.ret_type).unwrap(),
                    None => Type::Void,
                }
                .into(),
            ),
            evals_to: None,
            expression_types: Vec::new(),
        });
        for (param, _) in function.params.inner.inner.iter() {
            self.param(param);
        }
        if let Some(last_param) = function.params.last.as_ref() {
            self.param(last_param);
        }
        self.block(&function.body);
        self.scopes.borrow_mut().pop();
    }
}
