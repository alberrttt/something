pub trait TypeCheck<With> {
    fn type_check(&self, _: With) -> Result<(), TypeError>;
}
pub trait ResolveType<Ctx> {
    fn resolve_type(&self, ctx: Ctx) -> Type;
}
use something_ast::prelude::{block::Block, *};

use crate::{
    context::BlockCtx,
    prelude::{Type, TypeError},
};
impl TypeCheck<&mut BlockCtx> for Block {
    fn type_check(&self, ctx: &mut BlockCtx) -> Result<(), TypeError> {
        let mut nodes = self.0.iter().peekable();
        while let Some(node) = nodes.next() {
            let is_last_node = nodes.peek().is_none();
            match node {
                Node::Statement(stmt) => stmt.type_check((ctx, is_last_node))?,
                Node::Declaration(decl) => todo!(),
            }
        }
        Ok(())
    }
}
impl TypeCheck<(&mut BlockCtx, bool)> for &Statement {
    fn type_check(&self, (ctx, is_last_node): (&mut BlockCtx, bool)) -> Result<(), TypeError> {
        match self {
            Statement::Expression(_, _) => Ok(()),
            Statement::Return(_, expr, _) => {
                let return_type = Type::from(expr);
                if ctx.should_eval_to.ne(&return_type) {
                    Err(TypeError::MismatchedTypes {
                        expected: ctx.should_eval_to.clone(),
                        got: return_type,
                    })
                } else {
                    Ok(())
                }
            }
        }
    }
}
impl From<Block> for Type {
    /// the only thing this does is get the type of that the return statement gives
    fn from(block: Block) -> Self {
        for node in block.0.iter() {
            match node {
                Node::Statement(statement) => match statement {
                    Statement::Expression(_, _) => {}
                    Statement::Return(_, expression, _) => return Type::from(expression.clone()),
                },
                _ => todo!(),
            }
        }
        todo!()
    }
}
impl From<Statement> for Type {
    fn from(value: Statement) -> Self {
        match value {
            Statement::Return(_, expression, _) => expression.into(),
            Statement::Expression(expression, _) => Self::void(),
        }
    }
}
impl From<Binary> for Type {
    fn from(value: Binary) -> Self {
        (&value).into()
    }
}

impl From<&Binary> for Type {
    fn from(value: &Binary) -> Self {
        use something_ast::prelude::Operator::*;
        match value.operator {
            Plus | Minus | Multiply | Divide => Self::number(),
            PlusEqual | MinusEqual | MultiplyEqual | DivideEqual => Self::number(),
            EqualEqual | BangEqual => Self::boolean(),
            Greater | Less | GreaterEqual | LessEqual => Self::boolean(),
            _ => todo!(),
        }
    }
}
