use std::primitive;

use something_frontend::VariableDeclaration;
use something_frontend_tokenizer::list::List;

use crate::{
    context::{BlockCtx, Scope},
    prelude::TypeError,
    symbol::Symbol,
    traits::{ResolveType, TypeCheck},
};

impl TypeCheck<&mut BlockCtx> for VariableDeclaration {
    fn type_check(&self, ctx: &mut BlockCtx) -> Result<(), TypeError> {
        dbg!(self);
        let expr_type = self.value.resolve_type(ctx);
        match &self.type_annotation {
            Some((_, typename)) => {
                let typename = crate::primitives::Type::from(typename);
                if expr_type != typename {
                    return Err(TypeError::MismatchedTypes {
                        expected: typename,
                        got: expr_type,
                    });
                }
            }
            None => {}
        }
        let symbol: Symbol = (&self.name).into();
        ctx.set(symbol, expr_type);
        Ok(())
    }
}
use something_ast::ast;
#[test]
fn type_check_var() {
    let var_ast: List<VariableDeclaration> = ast!("let ident = 1;");
    let mut ctx = BlockCtx::default();
    for var in var_ast.iter() {
        var.type_check(&mut ctx).unwrap();
    }
    dbg!(ctx);
}
