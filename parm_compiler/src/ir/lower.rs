use std::collections::HashMap;

use crate::{ast::prelude::*, typechecker::Typechecker};

use super::{
    code::IRCode,
    function::IRFunction,
    registers::{Register, Registers},
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Lowering<'a> {
    pub registers: Registers,
    pub vars: HashMap<&'a str, u8>,
    pub tc: &'a Typechecker<'a>,
}

impl<'a> Lowering<'a> {
    pub fn lower(&mut self) -> IRFunction {
        let mut ir_function = IRFunction { code: vec![] };
        for item in &self.tc.source_file.ast {
            let Item::Function(fnc) = item else {
                continue;
            };
            if fnc.name.lexeme != "main" {
                continue;
            }
            for statement in &fnc.body.statements.inner {
                ir_function.code.extend(self.lower_statement(statement));
            }
        }
        ir_function
    }
    /// assuming that you've already allocated `into` if provided
    pub fn lower_expression(&mut self, expression: &Expression, into: u8) -> Vec<IRCode> {
        let mut ir_code = vec![];
        match expression {
            Expression::Number(number) => ir_code.push(IRCode::LoadValue {
                from: Box::new(Value::Float(number.value)),
                into,
            }),
            Expression::BinaryExpression(binary_expr) => {
                let lhs = self.registers.allocate().unwrap();
                ir_code.extend(self.lower_expression(&binary_expr.left, lhs));
                let rhs = self.registers.allocate().unwrap();
                ir_code.extend(self.lower_expression(&binary_expr.right, rhs));
                ir_code.push(IRCode::Add { lhs, rhs, into });
                self.registers.deallocate(lhs);
                self.registers.deallocate(rhs);
            }
            Expression::Call(call_expr) => {
                let Expression::Identifier(callee) = call_expr.callee.as_ref() else {
                    panic!()
                };

                let symbol = callee.symbol.as_ref().unwrap();
                if symbol.name != "println" {
                    panic!()
                }
                let arg = self.registers.allocate().unwrap();
                ir_code.extend(self.lower_expression(call_expr.arguments.collect_t()[0], arg));
                ir_code.push(IRCode::Print { value: arg });
            }
            Expression::Identifier(ident) => {
                let register = self.vars.get(ident.lexeme).unwrap();
                ir_code.push(IRCode::Move {
                    from: *register,
                    into,
                });
            }
            _ => {}
        }
        ir_code
    }
    pub fn lower_statement(&mut self, statement: &Statement<'a>) -> Vec<IRCode> {
        let mut ir_code = vec![];
        match statement {
            Statement::Let(let_stmt) => {
                let free_register = self.registers.allocate().unwrap();
                ir_code.extend(
                    self.lower_expression(
                        &let_stmt.initializer.as_ref().unwrap().expr,
                        free_register,
                    ),
                );
                self.vars.insert(let_stmt.ident.lexeme, free_register);
            }
            Statement::ExpressionWithSemi(expr) => {
                let free_register = self.registers.allocate().unwrap();
                ir_code.extend(self.lower_expression(&expr.expression, free_register));
                self.registers.deallocate(free_register);
            }
            _ => {
                panic!();
            }
        };
        ir_code
    }
}
