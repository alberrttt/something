#![feature(decl_macro)]
use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

use parm_ast::{parser::nodes::statement, prelude::*};
use symbol::Symbol;
use types::{FnSig, Type, F64, I32};
mod symbol;
mod traits;
mod types;
#[derive(Debug, Default)]
pub struct Scope<'a> {
    pub variables: HashMap<&'a str, Rc<RefCell<Symbol<'a>>>>,
}

impl<'a> Scope<'a> {
    /// O(n)
    pub fn get(&self, name: impl Into<&'a str>) -> Option<Rc<RefCell<Symbol<'a>>>> {
        self.variables.get(name.into()).cloned()
    }
}

#[derive(Debug)]
pub struct TypeCheckedSourceFile<'a> {
    pub source_file: SourceFile<'a>,
    pub typechecker: TypeChecker<'a>,
}

impl<'a> TypeCheckedSourceFile<'a> {
    pub fn new(source_file: SourceFile<'a>) -> Self {
        Self {
            source_file,
            typechecker: TypeChecker { scopes: vec![] },
        }
    }
    pub fn typecheck(&mut self) {
        let mut scope = Scope::default();
        for item in &self.source_file.ast {
            if let Item::Function(function) = item {
                let return_type = self
                    .typechecker
                    .ty_from_ty_expr(&function.ret_type)
                    .unwrap();
                let mut params: Vec<Type> = Vec::new();

                for param in function.params.inner.iter() {
                    let ty = &param.annotation.ty;
                    let ty = Type::numeric(ty);
                    params.push(ty.unwrap());
                }
                scope.variables.insert(
                    function.name.lexeme,
                    Rc::new(RefCell::new(Symbol {
                        declaration: Some(symbol::SymbolDeclaration::Function(function.clone())),
                        ty: Type::FnSig(FnSig {
                            params,
                            return_type: Box::new(return_type),
                        }),
                    })),
                );
            }
        }
        self.typechecker.scopes.push(scope);

        for item in &self.source_file.ast {
            self.typechecker.item(item);
        }
    }
}

#[derive(Debug)]
pub struct TypeChecker<'a> {
    pub scopes: Vec<Scope<'a>>,
}
/// the method to typecheck an ast node is just its name.
impl<'a> TypeChecker<'a> {
    /// typechecks an item

    pub fn item(&mut self, item: &Item<'a>) {
        match item {
            Item::Use(use_stmt) => self.use_stmt(use_stmt),
            Item::Function(function) => self.function(function),
            Item::Struct(struct_) => self.struct_(struct_),

            item => todo!("{:?}", item),
        }
        return;
    }
    pub fn scope<'b>(&'b self) -> &'b Scope<'a> {
        self.scopes.last().as_ref().unwrap()
    }
    pub fn get_symbol(&self, ident: &str) -> Option<Rc<RefCell<Symbol<'a>>>> {
        let mut scope = self.scope();
        loop {
            match scope.variables.get(ident) {
                Some(ty) => return Some(ty.clone()),
                None => {
                    scope = match self.scopes.get(self.scopes.len() - 2) {
                        Some(scope) => scope,
                        None => return None,
                    };
                }
            }
        }
    }

    pub fn expression<'b: 'a>(&mut self, expr: &Expression<'a>) -> Type {
        match expr {
            Expression::BinaryExpression(binary) => self.binary_expression(binary),
            Expression::Group(group) => self.expression(&group.paren.inner),
            Expression::Number(number) => {
                let value = number.value;
                let is_integer = value.fract() == 0.0;
                if is_integer {
                    Type::Numeric(types::Numeric::I32(I32::new()))
                } else {
                    Type::Numeric(types::Numeric::F64(F64::new()))
                }
            }
            Expression::Identifier(identifier) => {
                let symbol = self.get_symbol(identifier.lexeme).unwrap();
                let symbol = symbol.as_ref().borrow();
                symbol.ty.clone()
            }
            Expression::Call(call) => {
                todo!();
            }
            _ => todo!(),
        }
    }
    pub fn binary_expression(&mut self, binary: &BinaryExpression<'a>) -> Type {
        let lhs_ty = self.expression(&binary.left);
        let rhs_ty = self.expression(&binary.right);
        if lhs_ty != rhs_ty {
            todo!()
        } else {
            lhs_ty
        }
    }
    pub fn statement(&mut self, statement: &Statement<'a>) {
        match statement {
            Statement::Expression(expression) => {
                self.expression(expression);
            }
            Statement::ExpressionWithSemi((expression, _)) => {
                self.expression(expression);
            }
            Statement::Item(item) => {
                self.item(item);
            }
            Statement::Let(let_) => {
                self.variable(let_);
            }
        };
    }
    pub fn variable(&mut self, variable: &LetStmt<'a>) {
        let ty = self.expression(&variable.initializer.as_ref().unwrap().expr);
        let scope = self.scopes.last_mut().unwrap();

        scope.variables.insert(
            variable.ident.lexeme,
            Rc::new(RefCell::new(Symbol {
                declaration: Some(symbol::SymbolDeclaration::Variable(variable.clone())),
                ty,
            })),
        );
    }
    pub fn struct_(&mut self, struct_: &Struct) {}
    pub fn use_stmt(&mut self, use_stmt: &UseStatement<'a>) {}
    pub fn param(&mut self, param: &Param<'a>) {
        let ty = &param.annotation.ty;
        let ty = self.ty_from_ty_expr(ty).unwrap();

        let scope = self.scopes.last_mut().unwrap();
        scope.variables.insert(
            param.name.lexeme,
            Rc::new(RefCell::new(Symbol {
                declaration: Some(symbol::SymbolDeclaration::Param(param.clone())),
                ty,
            })),
        );
    }
    pub fn function(&mut self, function: &Function<'a>) {
        let parent = self.scopes.last_mut().unwrap();

        self.scopes.push(Scope::default());
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

    pub fn ty_from_ty_expr(&self, ty_expr: &TypeExpression<'a>) -> Result<Type, Box<dyn Error>> {
        let path = &ty_expr.path;
        let path = &path.segments.last;

        let ident = path.as_ref().unwrap();
        let ident = &ident.ident;

        let numeric = Type::numeric(ty_expr);
        if let Some(numeric) = numeric {
            return Ok(numeric);
        }

        let boolean = Type::boolean(ty_expr);
        if let Some(boolean) = boolean {
            return Ok(boolean);
        }

        if ident.lexeme == "void" {
            return Ok(Type::Void);
        }

        todo!()
    }
}
