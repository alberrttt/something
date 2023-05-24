use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq)]
pub struct NumberType {
    pub strict: Option<()>,
}
impl Debug for NumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}Number",
            if self.strict.is_some() { "Strict" } else { "" }
        )
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct StringType {
    pub strict: Option<()>,
}
impl Debug for StringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}String",
            if self.strict.is_some() { "Strict" } else { "" }
        )
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct BooleanType {
    pub strict: Option<()>,
}
impl Debug for BooleanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}Boolean",
            if self.strict.is_some() { "Strict" } else { "" }
        )
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct FunctionType {
    pub strict: Option<()>,
}
impl Debug for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}Function",
            if self.strict.is_some() { "Strict" } else { "" }
        )
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Number(NumberType),
    String(StringType),
    Boolean(BooleanType),
    Function(FunctionType),
}
impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => write!(f, "{:?}", arg0),
            Self::String(arg0) => write!(f, "{:?}", arg0),
            Self::Boolean(arg0) => write!(f, "{:?}", arg0),
            Self::Function(arg0) => write!(f, "{:?}", arg0),
        }
    }
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Type::*;
        match self {
            Number(_) => write!(f, "number"),
            String(_) => write!(f, "string"),
            Boolean(_) => write!(f, "boolean"),
            Function(_) => write!(f, "function"),
        }
    }
}
impl Type {
    pub fn is_strict(&self) -> bool {
        match self {
            Type::Number(n) => n.strict.is_some(),
            Type::String(n) => n.strict.is_some(),
            Type::Boolean(n) => n.strict.is_some(),
            Type::Function(n) => n.strict.is_some(),
        }
    }
    pub fn number() -> Self {
        Self::Number(NumberType { strict: None })
    }
    pub fn string() -> Self {
        Self::String(StringType { strict: None })
    }
    pub fn boolean() -> Self {
        Self::Boolean(BooleanType { strict: None })
    }
    pub fn function() -> Self {
        Self::Function(FunctionType { strict: None })
    }
}
