use std::io::IntoInnerError;

use parm_ast::parser::nodes::statement::variable::Initializer;

use crate::{
    expression::Expression,
    symbol::{InnerSymbol, Symbol, SymbolDeclaration},
    traits::{Check, TypeCheckResult},
    typechecker::Typechecker,
    AST,
};

#[derive(Clone, PartialEq)]
pub enum Statement<'a, 'b> {
    Expression(Expression<'a, 'b>),
    LetStatement(LetStatement<'a, 'b>),
}
impl<'a, 'b> std::fmt::Debug for Statement<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Expression(expr) => expr.fmt(f),
            Statement::LetStatement(stmt) => stmt.fmt(f),
        }
    }
}
impl<'a, 'b> Check<'a, 'b, Statement<'a, 'b>> for parm_ast::prelude::Statement<'a> {
    fn check(&'b self, tc: &mut Typechecker<'a, 'b>) -> TypeCheckResult<'a, 'b, Statement<'a, 'b>> {
        use parm_ast::prelude::Statement as ASTStatement;
        match self {
            ASTStatement::Let(stmt) => Ok(Statement::LetStatement(stmt.check(tc)?)),
            ASTStatement::Expression(expr) => Ok(Statement::Expression(expr.check(tc)?)),
            ASTStatement::ExpressionWithSemi(expr) => {
                Ok(Statement::Expression(expr.expression.check(tc)?))
            }
            _ => todo!(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub expression: Expression<'a, 'b>,
}
impl<'a, 'b> Check<'a, 'b, LetStatement<'a, 'b>> for parm_ast::prelude::LetStatement<'a> {
    fn check(
        &'b self,
        typechecker: &mut Typechecker<'a, 'b>,
    ) -> TypeCheckResult<'a, 'b, LetStatement<'a, 'b>> {
        
        let Initializer { eq: _, expr } = self.initializer.as_ref().unwrap();
        let expression = expr.check(typechecker)?;
        let name: &parm_ast::prelude::Identifier<'_> = &self.ident;
        let symbol = InnerSymbol {
            declaration: SymbolDeclaration::LetStatement(AST(self)),
            ty: expression.get_ty(),
            lexeme: name.lexeme,
        }
        .into_symbol();
        typechecker.mut_current_scope().push_symbol(symbol.clone());
        Ok(LetStatement { symbol, expression })
    }
}
impl<'a, 'b> LetStatement<'a, 'b> {
    pub fn get_declaration(
        &self,
    ) -> Option<AST<&parm_ast::parser::nodes::statement::variable::LetStatement<'a>>> {
        let inner = self.symbol.inner.borrow();
        let SymbolDeclaration::LetStatement(ref stmt) = inner.declaration else {
            return None;
        };
        Some(AST(stmt))
    }
}
