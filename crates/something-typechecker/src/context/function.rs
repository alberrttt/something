use std::{collections::HashMap, rc::Rc};

use something_frontend::{FunctionDeclaration, Ident};

use crate::types::{
    primitives::Primitive,
    sig::{FnSig, TypeSig},
};

use super::{BlockContext, Context};

#[derive(Debug, Clone, Default)]
pub struct FnContext {
    pub(crate) parameters: HashMap<Ident, TypeSig>,
    pub(crate) block: BlockContext,
    pub(crate) parent: Option<Rc<Context>>,
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
        ctx.block = BlockContext::from_ast_block(
            {
                BlockContext {
                    parent: Some(Rc::new(Context::Function(ctx.clone()))),
                    ..Default::default()
                }
            },
            &value.body,
            return_type,
        );
        ctx
    }
}
impl From<&FunctionDeclaration> for FnContext {
    fn from(value: &FunctionDeclaration) -> Self {
        let mut parameters: HashMap<Ident, TypeSig> = HashMap::new();
        for ((ty, name), _) in value.params.iter() {
            parameters.insert(name.clone(), Primitive::from(ty).into());
        }

        let parent = None;
        let return_type: Primitive = (&value.return_type.ty).into();
        let mut ctx = Self {
            parameters,
            block: BlockContext::default(),
            parent,
        };
        ctx.block = BlockContext::from_ast_block(
            {
                BlockContext {
                    parent: Some(Rc::new(Context::Function(ctx.clone()))),
                    ..Default::default()
                }
            },
            &value.body,
            return_type,
        );
        ctx
    }
}
