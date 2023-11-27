use std::marker::PhantomData;

use crate::vm::VM;

#[derive(Debug, Clone, PartialEq)]
pub enum Operand<'a> {
    Local(Option<ConstantRef<'a>>),
    Binary(BinaryOperand),

    Print,
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

#[test]
fn test() {
    let mut main = Function::new("main");
    let constant = main.add_value(Value::Number(1.0));
    main.add_operand(Operand::Local(constant));
    let const2 = main.add_value(Value::Number(2.0));
    main.add_operand(Operand::Local(const2));
    main.add_operand(Operand::Binary(BinaryOperand::Add));
    main.add_operand(Operand::Print);

    let mut vm = VM::default();

    vm.run(&main);
}
