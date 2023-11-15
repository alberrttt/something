use std::marker::PhantomData;

use crate::vm::Vm;

#[derive(Debug, Clone, PartialEq)]
pub enum Operand<'a> {
    Local(LocalName, ConstantRef<'a>),
    Binary(BinaryOperand, LocalName, ConstantRef<'a>),

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
impl LocalName {
    pub fn new(name: String) -> Self {
        LocalName { name }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantRef<'a> {
    pub constant: usize,
    pub value: PhantomData<&'a Function<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub constants: Vec<Value>,
    pub code: Vec<Operand<'a>>,
}

impl<'a> Function<'a> {
    pub fn new(name: &'a str) -> Self {
        Function {
            name,
            constants: Vec::new(),
            code: Vec::new(),
        }
    }
    pub fn add_value(&mut self, value: Value) -> ConstantRef<'a> {
        self.constants.push(value);

        ConstantRef {
            constant: self.constants.len() - 1,
            value: PhantomData,
        }
    }
    pub fn add_operand(&mut self, operand: Operand<'a>) {
        self.code.push(operand);
    }
}
impl<'a> Vm for Function<'a> {
    fn execute(&self) {
        
    }
}
#[test]
fn test() {
    let mut main = Function::new("main");
    let constant = main.add_value(Value::Number(1.0));
    main.add_operand(Operand::Local(LocalName::new("foo".into()), constant));
    main.add_operand(Operand::Print(LocalName::new("foo".into())));
}
