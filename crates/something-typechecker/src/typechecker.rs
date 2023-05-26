use std::collections::HashMap;

use something_ast::TopLevelNode;

use crate::prelude::*;
#[derive(Default, Debug, Clone)]
pub struct TypeChecker {
    pub file: FileScope,
}

impl TryFrom<Ast> for TypeChecker {
    type Error = TypeError;

    fn try_from(ast: Ast) -> Result<Self, Self::Error> {
        let mut file = FileScope::default();
        for node in ast.nodes {
            match node {
                TopLevelNode::FunctionDeclaration(function_declaration) => {
                    let block_ctx: BlockScope = (&function_declaration).try_into()?;
                    file.fns.push(FnCtx {
                        block: block_ctx,
                        name: function_declaration.name,
                    });
                }
            }
        }
        Ok(Self { file })
    }
}
