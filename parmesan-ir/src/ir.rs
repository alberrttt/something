#[derive(Debug, Clone, PartialEq)]
pub enum Operands {
    Local(LocalName, Value),
    Binary(BinaryOperand, LocalName, Value),
    
    Print(LocalName),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperand {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalName {
    pub name: String,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub code: Vec<Operands>,
}
