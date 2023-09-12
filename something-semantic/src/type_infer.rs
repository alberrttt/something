use something_ast::{
    ast::prelude::{Expression, VariableDeclaration},
    tokenizer::prelude::{lit_impl, Ident, Literal},
};

use crate::{error::TypeError, symbol::Type};
pub trait InferType {
    type Output = Result<Type, TypeError>;
    fn infer_type(&self) -> Self::Output;
}
pub trait InferLiteralType {
    type Output = Result<Type, TypeError>;
    fn infer_literal_type(&self) -> Self::Output;
}
impl InferLiteralType for VariableDeclaration {
    type Output = Result<Type, TypeError>;

    fn infer_literal_type(&self) -> Self::Output {
        if let Some(ta) = &self.type_annotation {
            ta.1.infer_literal_type()
        } else {
            self.expression.infer_literal_type()
        }
    }
}
impl InferLiteralType for Expression {
    type Output = Result<Type, TypeError>;

    fn infer_literal_type(&self) -> Self::Output {
        match self {
            Expression::Lit(literal) => literal.infer_literal_type(),

            _ => todo!(),
        }
    }
}
impl InferLiteralType for Literal {
    type Output = Result<Type, TypeError>;

    fn infer_literal_type(&self) -> Self::Output {
        match self.inner {
            something_ast::tokenizer::prelude::lit_impl::Inner::String(_) => todo!(),
            lit_impl::Inner::Float(n) => Ok(Type::Number),
            lit_impl::Inner::Boolean(_) => Ok(Type::Bool),
            lit_impl::Inner::Integer(i) => Ok(Type::Number),
        }
    }
}
impl InferLiteralType for Ident {
    fn infer_literal_type(&self) -> Self::Output {
        match self.name.as_str() {
            "number" => Ok(Type::Number),
            "bool" => Ok(Type::Bool),
            "void" => Ok(Type::Void),
            _ => Err(TypeError::Generic(
                "
                Unexpected type name
            ",
            )),
        }
    }
}

#[test]
// try to convert an ident into a type, and add test cases like int, random_string, etc.
fn test_infer_literal_type() {
    let ident = Ident::from("number");
    assert_eq!(ident.infer_literal_type().unwrap(), Type::Number);

    let ident = Ident::from("bool");
    assert_eq!(ident.infer_literal_type().unwrap(), Type::Bool);

    let ident = Ident::from("void");
    assert_eq!(ident.infer_literal_type().unwrap(), Type::Void);

    let ident = Ident::from("random_string");
    assert!(ident.infer_literal_type().is_err());
}
