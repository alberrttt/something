#![feature(decl_macro)]
use std::collections::HashMap;

use parm_ast::prelude::*;
use types::Type;
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
    pub fn expression(&mut self, binary: &Expression<'a>) {
        match binary {
            Expression::BinaryExpression(binary) => self.binary_expression(binary),
            Expression::Group(group) => self.expression(&group.paren.inner),
            _ => {}
        }
    }
    pub fn binary_expression(&mut self, binary: &BinaryExpression<'a>) {}
    pub fn statement(&mut self, statement: &Statement<'a>) {}
    pub fn struct_(&mut self, struct_: &Struct) {}
    pub fn use_stmt(&mut self, use_stmt: &UseStatement<'a>) {}
    pub fn param(&mut self, param: &Param<'a>) {
        let scope = self.scopes.last_mut().unwrap();
        let ty = &param.annotation.ty;
        let ty = Type::numeric(ty);
        scope.variables.insert(param.name.lexeme, ty);
    }
    pub fn function(&mut self, function: &Function<'a>) {
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
    }
}
