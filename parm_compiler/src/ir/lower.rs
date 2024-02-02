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
    pub fn lower_expression(
        &mut self,
        expression: &Expression,
        into: Option<u8>,
    ) -> (Vec<IRCode>, u8) {
        let mut ir_code = vec![];
        let mut result_register = into.unwrap_or(self.registers.allocate().unwrap());
        match expression {
            Expression::Number(number) => ir_code.push(IRCode::LoadValue {
                from: Box::new(Value::Float(number.value)),
                into: result_register,
            }),
            Expression::BinaryExpression(binary_expr) => {
                let (code, lhs) = self.lower_expression(&binary_expr.left, None);
                ir_code.extend(code);
                let (code, rhs) = self.lower_expression(&binary_expr.right, None);
                ir_code.extend(code);
                match binary_expr.operator {
                    BinaryOperator::Eq(_) => {
                        ir_code.push(IRCode::Reassign {
                            from: rhs,
                            into: lhs,
                        });
                        result_register = lhs;
                    }
                    BinaryOperator::Asterisk(_) => {
                        ir_code.push(IRCode::Mul {
                            lhs,
                            rhs,
                            into: result_register,
                        });
                    }
                    BinaryOperator::Minus(_) => {
                        ir_code.push(IRCode::Sub {
                            lhs,
                            rhs,
                            into: result_register,
                        });
                    }
                    BinaryOperator::Plus(_) => {
                        ir_code.push(IRCode::Add {
                            lhs,
                            rhs,
                            into: result_register,
                        });
                    }
                    _ => {}
                }
                dbg!(lhs);
                dbg!(rhs);
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
                let (result, arg) = self.lower_expression(call_expr.arguments.collect_t()[0], None);
                ir_code.extend(result);
                ir_code.push(IRCode::Print { value: arg });
            }
            Expression::Identifier(ident) => {
                let register = self.vars.get(ident.lexeme).unwrap();
                self.registers.deallocate(result_register);
                result_register = *register;
            }
            _ => {}
        }
        (ir_code, result_register)
    }
    pub fn lower_statement(&mut self, statement: &Statement<'a>) -> Vec<IRCode> {
        let mut ir_code = vec![];
        match statement {
            Statement::Let(let_stmt) => {
                let (code, reg) =
                    self.lower_expression(&let_stmt.initializer.as_ref().unwrap().expr, None);
                ir_code.extend(code);
                self.vars.insert(let_stmt.ident.lexeme, reg);
            }
            Statement::ExpressionWithSemi(expr) => {
                let (code, register) = self.lower_expression(&expr.expression, None);
                ir_code.extend(code);
                self.registers.deallocate(register);
            }
            _ => {
                panic!();
            }
        };
        ir_code
    }
}
