use crate::prelude::*;
use something_frontend::{Binary, Expression, Operator, VariableDeclaration};
use something_frontend_tokenizer::prelude::*;

pub trait InferType<T = Type> {
    fn infer_type(&self) -> T;
}
/// returns the annotation and the inferred type of the expr
impl InferType<(Type, Type)> for VariableDeclaration {
    fn infer_type(&self) -> (Type, Type) {
        let (_, ty) = self.type_annotation.clone().unwrap();
        let value_ty = self.value.infer_type();
        (ty.try_into().unwrap(), value_ty)
    }
}

impl InferType for Expression {
    fn infer_type(&self) -> Type {
        match self {
            Expression::Lit(lit) => lit.infer_type(),
            Expression::Binary(binary) => binary.infer_type(),
            Expression::Call(_) => todo!(),
            Expression::Ident(ident) => panic!(),
            Expression::Grouping(_) => todo!(),
            Expression::If(_) => todo!(),
            Expression::Block(_) => todo!(),
        }
    }
}
impl InferType for Literal {
    fn infer_type(&self) -> Type {
        use something_frontend_tokenizer::lit::lit_impl::Inner;
        match self.inner {
            Inner::Boolean(_) => Type::Boolean,
            Inner::Number(_) => Type::Number,
            Inner::String(_) => Type::String,
        }
    }
}
impl InferType for Binary {
    fn infer_type(&self) -> Type {
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
