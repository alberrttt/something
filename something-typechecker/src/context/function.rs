use std::{collections::HashMap, rc::Rc};

use something_ast::prelude::devprintln;
use something_frontend::{FunctionDeclaration, Ident};

use crate::{
    error::TypeError,
    traits::ResolveType,
    types::{
        primitives::Primitive,
        sig::{FnSig, TypeSig},
    },
};

use super::Context;

#[derive(Debug, Clone, Default)]
pub struct FnContext {
    pub(crate) parameters: HashMap<Ident, TypeSig>,
    pub(crate) parent: Option<Rc<Context>>,
    pub(crate) variables: HashMap<Ident, TypeSig>,
    pub(crate) return_type: Primitive,
}

impl TryFrom<&FnContext> for TypeSig {
    type Error = TypeError;

    fn try_from(value: &FnContext) -> Result<Self, Self::Error> {
        Ok(TypeSig::Fn(FnSig::try_from(value)?))
    }
}
macro_rules! return_if_error {
    ($result:expr) => {
        // if the result is an error, return it early
        if let Err(err) = $result {
            return Err(err);
        }
    };
}

impl FnContext {
    pub fn typecheck(mut self, value: &FunctionDeclaration) -> Result<Self, TypeError> {
        self.parameters = {
            let mut parameters: HashMap<Ident, TypeSig> = HashMap::new();
            for ((ty, name), _) in value.params.iter() {
                devprintln!("{}: {}", name, ty);
                parameters.insert(name.clone(), Primitive::from(ty).into());
            }
            parameters
        };

        let return_type: Primitive = (&value.return_type.ty).into();
        let mut ctx = Context::Function(self);
        for node in value.body.iter() {
            match node {
                something_ast::ast::Node::Statement(stmt) => match stmt {
                    something_ast::ast::statement::Statement::Expression((expr, _)) => {
                        return_if_error!(expr.resolve(&mut ctx))
                    }
                    something_ast::ast::statement::Statement::Return((_, expr, _)) => {
                        let expr_type: TypeSig = expr.resolve(&mut ctx)?;
                        if expr_type == (&return_type).into() {
                        } else {
                            panic!("Type mismatch")
                        }
                    }
                },
                something_ast::ast::Node::Declaration(decl) => match decl {
                    something_frontend::Declaration::Function(_) => todo!(),
                    something_frontend::Declaration::Var(var_decl) => {
                        var_decl.resolve(&mut ctx)?;
                    }
                },
            };
        }
        let Context::Function(ctx) = ctx else {
            unsafe {
                std::hint::unreachable_unchecked()
            }
        };
        Ok(ctx)
    }
}
