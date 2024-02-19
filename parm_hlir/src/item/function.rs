use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::prelude::*;
use crate::{
    statement::Statement,
    symbol::Symbol,
    ty::{function_ty::FunctionTy, Type},
    typechecker::Typechecker,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,

    pub statements: Vec<crate::statement::Statement<'a, 'b>>,
}

impl<'a, 'b> Check<'a, 'b> for Function<'a, 'b> {
    type Output = Self;
    type Ast = parm_ast::prelude::FunctionDeclaration<'a>;
    fn check(
        tyc: &mut Typechecker<'a, 'b>,
        function: &'b parm_ast::prelude::FunctionDeclaration<'a>,
    ) -> Self {
        let Typechecker {
            source_file: _,
            scopes_arena,
            current_scope: _,
        } = tyc;
        scopes_arena.push(Some(tyc.current_scope));

        let symbol = InnerSymbol {
            declaration: SymbolDeclaration::Function(AST(function)),
            ty: Type::None(PhantomData),
            lexeme: function.name.lexeme,
        };

        let symbol = Symbol {
            inner: Rc::new(RefCell::new(symbol)),
        };

        let ty = Rc::new(FunctionTy {
            symbol: symbol.clone(),
            return_ty: Type::None(PhantomData),
        });
        let mut statements = vec![];
        for statement in &function.body.statements.inner {
            statements.push(Statement::from_ast(tyc, statement))
        }

        Function {
            symbol: symbol.clone(),
            statements,
        }
    }
}
