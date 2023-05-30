use crate::prelude::*;
use something_frontend::{Binary, Expression, Operator, VariableDeclaration};
use something_frontend_tokenizer::prelude::*;

pub trait ResolveType<T = Type> {
    fn resolve_type(&self) -> T;
}
pub trait Scope<K = Ident, V = Type> {
    fn get(&self, key: &K) -> Option<V>;
    fn set(&mut self, key: &K, value: V);
}

/// returns the annotation and the inferred type of the expr
impl ResolveType<(Type, Type)> for VariableDeclaration {
    fn resolve_type(&self) -> (Type, Type) {
        let (_, ty) = self.type_annotation.clone().unwrap();
        let value_ty = self.value.resolve_type();
        (ty.try_into().unwrap(), value_ty)
    }
}

impl ResolveType for Expression {
    fn resolve_type(&self) -> Type {
        match self {
            Expression::Lit(lit) => lit.resolve_type(),
            Expression::Binary(binary) => binary.resolve_type(),
            Expression::Call(_) => todo!(),
            Expression::Ident(ident) => panic!(),
            Expression::Grouping(_) => todo!(),
            Expression::If(_) => todo!(),
            Expression::Block(_) => todo!(),
        }
    }
}
impl ResolveType for Literal {
    fn resolve_type(&self) -> Type {
        use something_frontend_tokenizer::lit::lit_impl::Inner;
        match self.inner {
            Inner::Boolean(_) => Type::Boolean,
            Inner::Number(_) => Type::Number,
            Inner::String(_) => Type::String,
        }
    }
}
impl ResolveType for Binary {
    fn resolve_type(&self) -> Type {
        match self.operator {
            Operator::Plus | Operator::Minus | Operator::Multiply | Operator::Divide => {
                Type::Number
            }
            Operator::EqualEqual
            | Operator::BangEqual
            | Operator::GreaterEqual
            | Operator::Greater
            | Operator::Less
            | Operator::LessEqual => Type::Boolean,
            _ => todo!(),
        }
    }
}

impl TryFrom<Ident> for Type {
    type Error = TypeError;
    fn try_from(ident: Ident) -> Result<Self, Self::Error> {
        match ident.name.as_str() {
            "boolean" => Ok(Type::Boolean),
            "number" => Ok(Type::Number),
            "string" => Ok(Type::String),
            "void" => Ok(Type::Void),
            tmp => Err(TypeError::IncorrectTypeName {
                expected: "boolean, number, string, or void",
                found: tmp.to_string(),
            }),
        }
    }
}
impl TryFrom<&Ident> for Type {
    type Error = TypeError;
    fn try_from(ident: &Ident) -> Result<Self, Self::Error> {
        match ident.name.as_str() {
            "boolean" => Ok(Type::Boolean),
            "number" => Ok(Type::Number),
            "string" => Ok(Type::String),
            "void" => Ok(Type::Void),
            tmp => Err(TypeError::IncorrectTypeName {
                expected: "boolean, number, string, or void",
                found: tmp.to_string(),
            }),
        }
    }
}
