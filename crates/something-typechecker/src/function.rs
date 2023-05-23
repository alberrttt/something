use std::{error::Error, rc::Rc};

use crate::{context::BlockCtx, prelude::*};
use colored::Colorize;
use something_ast::prelude::*;

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<(Type, Symbol)>,
    pub return_type: Type,
    pub fn_ast: Rc<FunctionDeclaration>,
}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.return_type == other.return_type
    }
}
impl From<&FunctionDeclaration> for Function {
    fn from(value: &FunctionDeclaration) -> Self {
        Self {
            params: value
                .params
                .1
                .iter()
                .map(|(ty, name)| (Type::from(ty.clone()), Symbol::from(name)))
                .collect(),
            return_type: Type::from(value.return_type.clone()),
            fn_ast: Rc::new(value.clone()),
        }
    }
}
impl From<&Rc<FunctionDeclaration>> for Function {
    fn from(value: &Rc<FunctionDeclaration>) -> Self {
        Self {
            params: value
                .params
                .1
                .iter()
                .map(|(ty, name)| (Type::from(ty.clone()), Symbol::from(name)))
                .collect(),
            return_type: Type::from(value.return_type.clone()),
            fn_ast: value.clone(),
        }
    }
}
impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}\n]",
            self.params
                .iter()
                .map(|f| { format!("\n  {}: {}", f.1, f.0) })
                .collect::<String>()
        )?;
        write!(f, " -> {}", self.return_type)
    }
}
impl TypeCheck<()> for Function {
    fn type_check(&self, _: ()) -> Result<(), TypeError> {
        let mut block_ctx = BlockCtx {
            should_eval_to: self.return_type.clone(),
            ..Default::default()
        };
        self.fn_ast.body.type_check(&mut block_ctx)?;
        Ok(())
    }
}
