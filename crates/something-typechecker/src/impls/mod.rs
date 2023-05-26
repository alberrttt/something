use crate::prelude::TypeCheck;

mod fn_decl;
mod to_type;
mod typecheck;
mod var;

use crate::prelude::*;
impl TypeCheck<&mut BlockCtx, ()> for Node {
    fn type_check(&self, ctx: &mut BlockCtx) {
        match self {
            Self::Declaration(declaration) => declaration.type_check(ctx),
            Self::Statement(statement) => statement.type_check(ctx),
        }
    }
}
impl TypeCheck<&mut BlockCtx, ()> for Statement {
    fn type_check(&self, with: &mut BlockCtx) {
        match self {
            Statement::Expression(expr, _) => expr.type_check(&*with),
            Statement::Return(_, _, _) => todo!(),
        }
    }
}
impl TypeCheck<&mut BlockCtx, ()> for Declaration {
    fn type_check(&self, with: &mut BlockCtx) {
        match self {
            Declaration::Function(_) => todo!(),
            Declaration::Var(var) => match var.type_check(with) {
                Ok(ok) => {}
                Err(err) => {
                    println!("{}", err);
                    println!("node: {var:?}");
                    panic!();
                }
            },
        }
    }
}
