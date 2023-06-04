use std::fmt::{Debug, Display};

use something_frontend::Ident;

#[derive(Clone, PartialEq, Default)]
pub enum Primitive {
    Number,
    String,
    Boolean,
    #[default]
    Void,
}
impl Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Primitive<{}>",
            match self {
                Primitive::Number => "Number",
                Primitive::String => "String",
                Primitive::Boolean => "Boolean",
                Primitive::Void => "Void",
            }
        )
    }
}
impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Primitive::*;
        match self {
            Number => write!(f, "number"),
            String => write!(f, "string"),
            Boolean => write!(f, "boolean"),
            Void => write!(f, "void"),
        }
    }
}
impl From<Ident> for Primitive {
    fn from(value: Ident) -> Self {
        use Primitive::*;
        match value.name.as_str() {
            "number" => Number,
            "string" => String,
            "boolean" => Boolean,
            "void" => Void,
            _ => unreachable!(),
        }
    }
}

impl From<&Ident> for Primitive {
    fn from(value: &Ident) -> Self {
        use Primitive::*;
        match value.name.as_str() {
            "number" => Number,
            "string" => String,
            "boolean" => Boolean,
            "void" => Void,
            _ => unreachable!(),
        }
    }
}
