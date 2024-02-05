use crate::ast::prelude::*;

use super::{code::IRCode, lower::Lowering, registers::RegisterRef, value::Value};

impl<'a> Expression<'a> {
    pub fn lower(
        &self,
        lowering: &mut Lowering<'a>,
        target_register: Option<RegisterRef>,
    ) -> (Vec<IRCode>, RegisterRef) {
        let mut target_register = target_register.unwrap_or_else(|| RegisterRef {
            register: lowering.registers.allocate().unwrap(),
            is_value: false,
        });
        let mut code: Vec<IRCode> = vec![];
        match &self {
            Expression::BinaryExpression(bin) => {
                let (bin_code, bin_register) = bin.lower(lowering, Some(target_register));
                code.extend(bin_code);
                target_register = bin_register;
            }
            Expression::Identifier(identifier) => {
                let register = lowering.vars.get(identifier.lexeme).unwrap();
                target_register.register = *register;
                target_register.is_value = false;
            }
            Expression::Number(number) => {
                code.push(IRCode::LoadValue {
                    from: Box::new(Value::Float(number.value)),
                    into: target_register.register,
                });
                target_register.is_value = true;
            }
            Expression::Call(call) => {
                let Expression::Identifier(ident) = &call.callee.as_ref() else {
                    panic!()
                };
                let symbol = ident.get_symbol().unwrap();
                if symbol.name != "println" {
                    return (code, target_register);
                }
                let (arg_code, arg_register) = call.arguments.collect_t()[0].lower(lowering, None);
                code.extend(arg_code);
                code.push(IRCode::Print {
                    value: arg_register.register,
                });
            }
            _ => {
                todo!()
            }
        }
        (code, target_register)
    }
}
impl<'a> BinaryExpression<'a> {
    pub fn lower(
        &self,
        lowering: &mut Lowering<'a>,
        target_register: Option<RegisterRef>,
    ) -> (Vec<IRCode>, RegisterRef) {
        let mut target_register = target_register.unwrap_or_else(|| RegisterRef {
            register: lowering.registers.allocate().unwrap(),
            is_value: false,
        });
        let mut code: Vec<IRCode> = vec![];
        if let BinaryOperator::Eq(_) = self.operator {
            let (left_code, left_register) = self.left.lower(lowering, None);
            let (right_code, right_register) = self.right.lower(lowering, Some(left_register));
            code.extend(left_code);
            code.extend(right_code);
            if left_register.is_value {
                lowering.registers.deallocate(left_register.register);
            }
            if right_register.is_value {
                lowering.registers.deallocate(right_register.register);
            }
            return (code, target_register);
        }
        let (left_code, left_register) = self.left.lower(lowering, None);
        let (right_code, right_register) = self.right.lower(lowering, None);
        code.extend(left_code);
        code.extend(right_code);
        if left_register.is_value {
            lowering.registers.deallocate(left_register.register);
        }
        if right_register.is_value {
            lowering.registers.deallocate(right_register.register);
        }
        match self.operator {
            BinaryOperator::Plus(_) => {
                code.push(IRCode::Add {
                    lhs: left_register.register,
                    rhs: right_register.register,
                    into: target_register.register,
                });
            }
            BinaryOperator::Minus(_) => {
                code.push(IRCode::Sub {
                    lhs: left_register.register,
                    rhs: right_register.register,
                    into: target_register.register,
                });
            }
            BinaryOperator::Asterisk(_) => {
                code.push(IRCode::Mul {
                    lhs: left_register.register,
                    rhs: right_register.register,
                    into: target_register.register,
                });
            }
            BinaryOperator::Slash(_) => {
                code.push(IRCode::Div {
                    lhs: left_register.register,
                    rhs: right_register.register,
                    into: target_register.register,
                });
            }

            _ => {
                todo!()
            }
        }
        (code, target_register)
    }
}
impl<'a> Statement<'a> {
    pub fn lower(&self, lowering: &mut Lowering<'a>) -> Vec<IRCode> {
        let mut code = vec![];
        match &self {
            Statement::ExpressionWithSemi(ExpressionWithSemi { expression, semi }) => {
                let (ir, reg) = expression.lower(lowering, None);
                code.extend(ir);
                lowering.registers.deallocate(reg.register);
            }
            Statement::Let(stmt) => {
                let initializer = &stmt.initializer.as_ref().unwrap().expr;
                let register = lowering.registers.allocate().unwrap();
                let (mut ir, reg) = initializer.lower(
                    lowering,
                    Some(RegisterRef {
                        register,
                        is_value: true,
                    }),
                );
                if let Expression::Identifier(ident) = &initializer {
                    ir.push(IRCode::Move {
                        from: reg.register,
                        into: register,
                    })
                }
                code.extend(ir);
                lowering.vars.insert(stmt.ident.lexeme, reg.register);
            }
            _ => {
                todo!()
            }
        };
        code
    }
}
