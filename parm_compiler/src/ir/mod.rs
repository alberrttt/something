use crate::typechecker::Scope;
use ast::*;
use prelude::*;
pub mod function;

pub mod ir;
pub mod ir_scope;
pub mod prelude;
pub mod state;

pub struct LoweringCtx<'a, 'b: 'a> {
    pub type_checker: &'b TypeChecker<'a, 'b>,
    pub state: State,
    // the stack should hopefully never be more than 2^32 bytes
    pub stack_index: u32,
    pub current_scope: &'b Scope<'a, 'b>,
}

impl<'a, 'b> LoweringCtx<'a, 'b> {
    pub fn loop_sub_scopes(scope: &Scope<'a, 'b>) {
        for sub in &scope.scopes {
            let mut sub = sub.borrow_mut();
            sub.current_sub_scope = 0;
            Self::loop_sub_scopes(&sub);
        }
    }
    pub fn new(type_checker: &'b TypeChecker<'a, 'b>) -> Self {
        type_checker.scope.borrow_mut().current_sub_scope = 0;
        Self::loop_sub_scopes(&type_checker.scope.borrow());
        Self {
            type_checker,
            state: State::default(),
            stack_index: 0,
            current_scope: type_checker.current_scope(),
        }
    }

    pub fn lower_fn(
        &mut self,
        function: &'b Function<'a>,
        scope: &mut IRScope<'a, 'b, '_>,
    ) -> FunctionIR<'_> {
        let mut function_ir = FunctionIR::new(function.name.lexeme);

        for stmt in function.body.statements.inner.iter() {
            let code = self.lower_stmt(&function_ir, scope, stmt);
            function_ir.code.extend(code);
        }
        function_ir.code.append(&mut scope.epilogue);
        function_ir
    }
    pub fn inc_stack(&mut self, size: u32) -> u32 {
        let stack_index = self.stack_index;
        self.stack_index += size;
        stack_index
    }

    pub fn lower_stmt(
        &mut self,
        function_ir: &FunctionIR<'_>,
        scope: &mut IRScope<'a, 'b, '_>,
        stmt: &Statement<'a>,
    ) -> Vec<IR> {
        let mut code: Vec<IR> = Vec::new();
        match stmt {
            Statement::Let(let_statement) => {
                let mut statement_code = self.lower_let_stmt(function_ir, scope, let_statement);
                code.extend(statement_code);
            }
            Statement::ExpressionWithSemi(stmt) => {
                let (mut statement_code, register) = self.lower_expression(&stmt.expression, scope);
                if let RegIdx::None = register {
                } else {
                    self.state.registers.free(register).unwrap();
                }
                code.extend(statement_code);
            }
            _ => {}
        }
        code
    }

    pub fn lower_expression(
        &mut self,
        expr: &Expression,
        scope: &mut IRScope,
    ) -> (Vec<IR>, RegIdx) {
        let mut code = Vec::new();
        let register = self.state.registers.get_unused().unwrap();
        match expr {
            Expression::Number(number) => {
                code.push(IR::Move {
                    from: IRValue::Float(number.value),
                    into: register,
                });
            }
            Expression::Call(call) => {}
            _ => {}
        }
        return (code, register);
    }
    pub fn lower_let_stmt(
        &mut self,
        function_ir: &FunctionIR<'_>,
        scope: &mut IRScope<'a, 'b, '_>,
        stmt: &LetStatement<'a>,
    ) -> Vec<IR> {
        let mut code: Vec<IR> = Vec::new();
        let symbol = scope.get(stmt.ident.lexeme);
        let symbol = symbol.as_ref().unwrap();
        let symbol = symbol.borrow();
        let ty = symbol.ty.as_ref();
        if let Type::Number(_) = ty {
        } else {
            todo!()
        }
        let value = &stmt.initializer.as_ref().unwrap().expr;
        let Expression::Number(number) = value else {
            todo!()
        };
        let (ir, register) = self.lower_expression(value, scope);
        code.extend(ir);

        scope
            .variables
            .insert(stmt.ident.lexeme, ir_scope::Location::Register(register));
        code
    }
}
