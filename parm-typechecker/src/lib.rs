#![feature(decl_macro)]
use std::collections::HashMap;

use parm_ast::prelude::*;
use types::{Type, F64, I32};
mod traits;
mod types;
#[derive(Debug, Default)]
pub struct Scope<'a> {
    pub variables: HashMap<&'a str, Type>,
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
        self.typechecker.scopes.push(Scope::default());
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
            Item::Statement(state) => self.statement(state),
            Item::Comment(_) => {}
            item => todo!("{:?}", item),
        }
    }
    pub fn scope(&self) -> &Scope {
        self.scopes.last().as_ref().unwrap()
    }
    pub fn get_variable(&self, ident: &str) -> Option<&Type> {
        let mut scope = self.scope();
        loop {
            match scope.variables.get(ident) {
                Some(ty) => return Some(ty),
                None => {
                    scope = match self.scopes.get(self.scopes.len() - 2) {
                        Some(scope) => scope,
                        None => return None,
                    };
                }
            }
        }
    }
    pub fn expression(&mut self, expr: &Expression<'a>) -> Type {
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
                let ty = self.get_variable(identifier.lexeme).unwrap();
                ty.clone()
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
            Statement::Expression(expression) => self.expression(expression),
            Statement::ExpressionWithSemi((expression, _)) => self.expression(expression),
            Statement::Use(_) => todo!(),
        };
    }
    pub fn variable(&mut self, variable: &Variable<'a>) {
        let ty = self.expression(&variable.initializer.as_ref().unwrap().expr);
        let scope = self.scopes.last_mut().unwrap();

        scope.variables.insert(variable.ident.lexeme, ty);
    }
    pub fn struct_(&mut self, struct_: &Struct) {}
    pub fn use_stmt(&mut self, use_stmt: &UseStatement<'a>) {}
    pub fn param(&mut self, param: &Param<'a>) {
        let scope = self.scopes.last_mut().unwrap();
        let ty = &param.annotation.ty;
        let ty = Type::numeric(ty);
        scope.variables.insert(param.name.lexeme, ty);
    }
    pub fn function(&mut self, function: &Function<'a>) {
        let parent = self.scopes.last_mut().unwrap();
        parent.variables.insert(
            function.name.lexeme,
            Type::Numeric(types::Numeric::I32(I32::new())),
        );

        self.scopes.push(Scope::default());
        for (param, _) in function.params.inner.inner.iter() {
            self.param(param);
        }
        if let Some(last_param) = function.params.last.as_ref() {
            self.param(last_param);
        }
        for item in function.body.iter() {
            self.item(item);
        }
        self.scopes.pop();
    }
}
