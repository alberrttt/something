use std::cell::UnsafeCell;

use crate::ir::{Function, Operand, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Flags {
    pub debug: bool,
    pub each_ir: bool,
}
impl Default for Flags {
    fn default() -> Self {
        Self {
            debug: true,
            each_ir: false,
        }
    }
}
#[derive(Debug, Default)]
pub struct VM<'a> {
    pub stack: UnsafeCell<Vec<Value>>,
    pub code: Vec<Operand<'a>>,
    pub call_stack: Vec<UnsafeCell<CallFrame<'a>>>,
    pub flags: Flags,
    pub ip: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallFrame<'a> {
    pub function: &'a Function<'a>,
    pub ip: usize,
    pub stack_base: usize,
}
impl<'a> VM<'a> {
    pub fn run(&mut self, function: &'a Function) {
        self.call_stack.push(
            CallFrame {
                function,
                ip: 0,
                stack_base: 0,
            }
            .into(),
        );

        let mut ip = 0;
        let mut current_frame = self.current_frame();
        let mut stack = &self.stack;
        while ip < unsafe { &*current_frame.get() }.function.code.len() {
            let current_frame = unsafe { &*current_frame.get() };
            let operand = &current_frame.function.code[ip];
            let stack = unsafe { &mut *stack.get() };
            match operand {
                Operand::Local(value) => {
                    stack.push(current_frame.function.constants[value.constant].clone());
                }
                Operand::Binary(op) => {
                    let Value::Number(right) = stack.pop().unwrap() else {
                        unsafe { std::hint::unreachable_unchecked() }
                    };
                    let Value::Number(left) = stack.pop().unwrap() else {
                        unsafe { std::hint::unreachable_unchecked() }
                    };

                    match op {
                        crate::ir::BinaryOperand::Add => stack.push(Value::Number(left + right)),
                        crate::ir::BinaryOperand::Sub => todo!(),
                        crate::ir::BinaryOperand::Mul => todo!(),
                        crate::ir::BinaryOperand::Div => todo!(),
                        crate::ir::BinaryOperand::Mod => todo!(),
                        crate::ir::BinaryOperand::Pow => todo!(),
                        crate::ir::BinaryOperand::Eq => todo!(),
                        crate::ir::BinaryOperand::Neq => todo!(),
                        crate::ir::BinaryOperand::Lt => todo!(),
                        crate::ir::BinaryOperand::Gt => todo!(),
                        crate::ir::BinaryOperand::Leq => todo!(),
                        crate::ir::BinaryOperand::Geq => todo!(),
                        crate::ir::BinaryOperand::And => todo!(),
                        crate::ir::BinaryOperand::Or => todo!(),
                    }
                }
                Operand::Print => {
                    let value = stack.pop().unwrap();

                    println!("{:?}", value);
                }
            }

            ip += 1;
        }
    }

    pub fn current_frame(&self) -> &UnsafeCell<CallFrame<'a>> {
        self.call_stack.last().unwrap()
    }
}
