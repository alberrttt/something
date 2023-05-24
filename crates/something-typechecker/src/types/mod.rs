#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberType {
    pub strict: Option<()>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringType {
    pub strict: Option<()>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanType {
    pub strict: Option<()>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    pub strict: Option<()>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Number(NumberType),
    String(StringType),
    Boolean(BooleanType),
    Function(FunctionType),
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
