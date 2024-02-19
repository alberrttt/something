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

use self::traits::TypeCheckResult;

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,

    pub statements: Vec<crate::statement::Statement<'a, 'b>>,
}

impl<'a, 'b> Check<'a, 'b, Function<'a, 'b>> for parm_ast::prelude::FunctionDeclaration<'a> {
    fn check(&'b self, tyc: &mut Typechecker<'a, 'b>) -> TypeCheckResult<'a, 'b, Function<'a, 'b>> {
        let Typechecker {
            source_file: _,
            scopes_arena,
            current_scope: _,
        } = tyc;
        scopes_arena.push(Some(tyc.current_scope));

        let symbol = InnerSymbol {
            declaration: SymbolDeclaration::Function(AST(self)),
            ty: Type::None(PhantomData),
            lexeme: self.name.lexeme,
        };

        let symbol = Symbol {
            inner: Rc::new(RefCell::new(symbol)),
        };

        let ty = Rc::new(FunctionTy {
            symbol: symbol.clone(),
            return_ty: Type::None(PhantomData),
        });
        let mut statements = vec![];
        for statement in &self.body.statements.inner {
            statements.push(statement.check(tyc)?)
        }

        Ok(Function {
            symbol: symbol.clone(),
            statements,
        })
    }
}
