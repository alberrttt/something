mod to_type;
mod typecheck;
use std::rc::Rc;

use something_frontend::VariableDeclaration;

use crate::{prelude::BlockCtx, traits::TypeCheck};

impl<'a> TypeCheck<&'a mut BlockCtx> for VariableDeclaration {
    fn type_check(&self, with: &'a mut BlockCtx) {
        if self.type_annotation.is_some() {
            annotated(with, self);
        } else {
            unannotated(with, self);
        }
    }
}
#[inline(always)]
fn annotated(ctx: &mut BlockCtx, var_decl: &VariableDeclaration) {
    let type_annotation = {
        let (_, type_annotation) = var_decl.type_annotation.as_ref().unwrap();
        type_annotation.type_check(())
    };

    let ty = var_decl.value.type_check(&*ctx);
    if ty != type_annotation {
        todo!("type mismatch")
    }
    let name = var_decl.name.clone();
    ctx.vars.insert(name, ty);
    dbg!(&ctx.vars);
}

#[inline(always)]
fn unannotated(ctx: &mut BlockCtx, var_decl: &VariableDeclaration) {}
