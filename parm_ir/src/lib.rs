use ast::*;
use prelude::*;
mod function;
mod ir;
mod prelude;
mod state;

pub struct LoweringCtx<'a, 'b: 'a> {
    pub type_checker: &'b TypeChecker<'a, 'b>,
    pub state: State,
    // the stack should hopefully never be more than 2^32 bytes
    pub stack_index: u32,
    pub scope_idx: usize,
}

impl<'a, 'b> LoweringCtx<'a, 'b> {
    pub fn new(type_checker: &'b TypeChecker<'a, 'b>) -> Self {
        Self {
            type_checker,
            state: State::default(),
            stack_index: 0,
            scope_idx: 0,
        }
    }

    pub fn lower_fn(&mut self, function: &'b Function<'a>) -> FunctionIR<'_> {
        let mut function_ir = FunctionIR::new(function.name.lexeme);

        self.scope_idx += 1;

        for stmt in function.body.statements.inner.iter() {
            let code = self.lower_stmt(&mut function_ir, stmt);
            function_ir.code.extend(code);
        }

        self.scope_idx -= 1;

        function_ir
    }
    pub fn inc_stack(&mut self, size: u32) -> u32 {
        let stack_index = self.stack_index;
        self.stack_index += size;
        stack_index
    }

    pub fn lower_stmt(&mut self, function_ir: &FunctionIR<'_>, stmt: &Statement<'a>) -> Vec<IR> {
        let mut code: Vec<IR> = Vec::new();
        match stmt {
            Statement::Let(let_statement) => {
                let mut statement_code = self.lower_let_stmt(function_ir, let_statement);
                code.extend(statement_code);
            }
            _ => {}
        }
        code
    }
    pub fn lower_let_stmt(
        &mut self,
        function_ir: &FunctionIR<'_>,
        stmt: &LetStatement<'a>,
    ) -> Vec<IR> {
        let mut code: Vec<IR> = Vec::new();
        let scopes = self.type_checker.scopes.borrow();
        let scope = scopes.get(self.scope_idx).unwrap();
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
        self.inc_stack(8);

        code.push(IR::Push {
            value: IRValue::Constant(number.value),
        });
        code
    }
}
