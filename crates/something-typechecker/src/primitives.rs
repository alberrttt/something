use std::rc::Rc;

use crate::symbol::Symbol;
use something_ast::prelude::{return_type::ReturnType, *};
#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<(Type, Symbol)>,
    pub return_type: Type,
    pub fn_ast: Rc<FunctionDeclaration>,
}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.return_type == other.return_type
    }
}
impl From<&FunctionDeclaration> for Function {
    fn from(value: &FunctionDeclaration) -> Self {
        Self {
            params: value
                .params
                .1
                .iter()
                .map(|(ty, name)| (Type::from(ty.clone()), Symbol::from(name)))
                .collect(),
            return_type: Type::from(value.return_type.clone()),
            fn_ast: Rc::new(value.clone()),
        }
    }
}
impl From<&Rc<FunctionDeclaration>> for Function {
    fn from(value: &Rc<FunctionDeclaration>) -> Self {
        Self {
            params: value
                .params
                .1
                .iter()
                .map(|(ty, name)| (Type::from(ty.clone()), Symbol::from(name)))
                .collect(),
            return_type: Type::from(value.return_type.clone()),
            fn_ast: value.clone(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number(Number),
    String(String),
    Boolean(Boolean),
    Void(Void),
    Function(Box<Function>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {}

#[derive(Debug, Clone, PartialEq)]
pub struct String {}

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {}

#[derive(Debug, Clone, PartialEq)]
pub struct Void {}
impl From<Literal> for Type {
    fn from(value: Literal) -> Self {
        match value.inner {
            lit_impl::Inner::Boolean(_) => Type::Boolean(Boolean {}),
            lit_impl::Inner::Number(_) => Type::Number(Number {}),
            lit_impl::Inner::String(_) => Type::String(String {}),
        }
    }
}
impl From<Ident> for Type {
    fn from(value: Ident) -> Self {
        (&value).into()
    }
}

impl From<&Ident> for Type {
    fn from(value: &Ident) -> Self {
        let contents = value.name.as_ref();
        match contents {
            "number" => Type::Number(Number {}),
            "string" => Type::String(String {}),
            "bool" => Type::Boolean(Boolean {}),
            "void" => Type::Void(Void {}),
            tmp => panic!("unexpected {tmp:?}"),
        }
    }
}

impl From<ReturnType> for Type {
    fn from(value: ReturnType) -> Self {
        value.ty.into()
    }
}
