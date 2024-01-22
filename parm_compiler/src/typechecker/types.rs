use std::{fmt::Display, rc::Rc};

use crate::ast::prelude::*;
// a 64 bit float
#[derive(Debug, Clone, PartialEq)]
pub struct Number {}
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "number")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct String {}
impl Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "string")
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number(Number),
    Boolean,
    String(String),
    Void,
    FnSig(FnSig),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Number(num) => write!(f, "{}", num),
            Type::Boolean => write!(f, "bool"),
            Type::String(string) => write!(f, "{}", string),
            Type::Void => write!(f, "void"),
            Type::FnSig(fn_sig) => write!(
                f,
                "fn({}) -> {}",
                fn_sig
                    .params
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                fn_sig.return_type
            ),
            _ => panic!(),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct FnSig {
    pub params: Vec<Type>,
    pub return_type: Rc<Type>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionSig {}

impl Type {}

impl<'a> TryFrom<&'a TypeExpression<'a>> for Type {
    type Error = ();

    fn try_from(value: &'a TypeExpression<'a>) -> Result<Self, Self::Error> {
        let path = &value.path;
        let path = &path.segments.last;

        let ident = path.as_ref().unwrap();
        match ident.ident.lexeme {
            "string" => return Ok(Type::String(String {})),
            "number" => return Ok(Type::Number(Number {})),
            "boolean" => return Ok(Type::Boolean),
            "void" => return Ok(Type::Void),
            _ => {}

            lexeme => {}
        };
        todo!("{}", value.tree())
    }
}
