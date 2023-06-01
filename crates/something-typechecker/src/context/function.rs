use std::{collections::HashMap, rc::Rc};

use something_frontend::{FunctionDeclaration, Ident};

use crate::{
    traits::ResolveType,
    types::{
        primitives::Primitive,
        sig::{FnSig, TypeSig},
    },
};

use super::{BlockContext, Context};

#[derive(Debug, Clone, Default)]
pub struct FnContext {
    pub(crate) parameters: HashMap<Ident, TypeSig>,
    pub(crate) parent: Option<Rc<Context>>,
    pub(crate) variables: HashMap<Ident, TypeSig>,
}

impl From<&FnContext> for FnSig {
    fn from(value: &FnContext) -> Self {
        let mut params = Vec::new();
        for (_, ty) in value.parameters.iter() {
            params.push(ty.clone());
        }
        let ret = TypeSig::Primitive(Primitive::Void);
        (params, Box::new(ret))
    }
}
impl FnContext {
    pub fn from_fn_decl(mut ctx: Self, value: &FunctionDeclaration) -> Self {
        let mut parameters: HashMap<Ident, TypeSig> = HashMap::new();
        for ((ty, name), _) in value.params.iter() {
            parameters.insert(name.clone(), Primitive::from(ty).into());
        }

        let return_type: Primitive = (&value.return_type.ty).into();
        ctx.parameters = parameters;
        let mut ctx = Context::Function(ctx);
        for node in value.body.iter() {
            match node {
                something_ast::Node::Statement(stmt) => match stmt {
                    something_frontend::Statement::Expression(expr, _) => {
                        expr.resolve(&mut ctx);
                    }
                    something_frontend::Statement::Return(_, expr, _) => {
                        let expr_type: TypeSig = expr.resolve(&mut ctx);
                        if expr_type == (&return_type).into() {
                        } else {
                            panic!("Type mismatch")
                        }
                    }
                },
                something_ast::Node::Declaration(decl) => match decl {
                    something_frontend::Declaration::Function(_) => todo!(),
                    something_frontend::Declaration::Var(var_decl) => {
                        var_decl.resolve(&mut ctx);
                    }
                },
            };
        }
        let Context::Function(ctx) = ctx else {
            unsafe {
                std::hint::unreachable_unchecked()
            }
        };
        ctx
    }
}
impl From<&FunctionDeclaration> for FnContext {
    fn from(value: &FunctionDeclaration) -> Self {
        Self::from(value)
    }
}
