use something_ast::tokenizer::prelude::Ident;
use something_common::Result::{self, *};

use crate::{error::TypeError, symbol::Type};
pub trait InferType {
    type Output = Result<Type, TypeError>;
    fn infer_type(&self) -> Self::Output;
}

impl InferType for Ident {
    fn infer_type(&self) -> Self::Output {
        match self.name.as_str() {
            "int" => Ok(Type::Int),
            "float" => Ok(Type::Float),
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
fn test_infer_type() {
    let ident = Ident::from("int");
    assert_eq!(ident.infer_type().unwrap(), Type::Int);
    
    let ident = Ident::from("float");
    assert_eq!(ident.infer_type().unwrap(), Type::Float);

    let ident = Ident::from("bool");
    assert_eq!(ident.infer_type().unwrap(), Type::Bool);

    let ident = Ident::from("void");
    assert_eq!(ident.infer_type().unwrap(), Type::Void);

    let ident = Ident::from("random_string");
    assert!(ident.infer_type().is_err());
}
