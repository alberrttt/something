use std::io::IntoInnerError;

use parm_ast::parser::nodes::declaration::variable::Initializer;

use crate::{
    expression::Expression,
    symbol::{InnerSymbol, Symbol, SymbolDeclaration},
    typechecker::Typechecker,
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
impl<'a, 'b> Statement<'a, 'b> {
    pub fn from_ast(
        typechecker: &mut Typechecker<'a, 'b>,
        statement: &'b parm_ast::prelude::Statement<'a>,
    ) -> Self {
        use parm_ast::prelude::Statement as ASTStatement;
        match statement {
            ASTStatement::Let(stmt) => {
                Statement::LetStatement(LetStatement::from_ast(typechecker, stmt))
            }
            ASTStatement::Expression(expr) => Statement::Expression(Expression::check_ast(expr)),
            _ => todo!(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub expression: Expression<'a, 'b>,
}

impl<'a, 'b> LetStatement<'a, 'b> {
    pub fn from_ast(
        typechecker: &mut Typechecker<'a, 'b>,
        statement: &'b parm_ast::prelude::LetStatement<'a>,
    ) -> Self {
        let Initializer { eq: _, expr } = statement.initializer.as_ref().unwrap();
        let expression = Expression::check_ast(expr);
        let name: &parm_ast::prelude::Identifier<'_> = &statement.ident;
        let id = typechecker.symbols_arena.symbols.len();
        let symbol = InnerSymbol {
            id,
            declaration: SymbolDeclaration::LetStatement(statement),
            ty: expression.get_ty(),
            lexeme: name.lexeme,
            tc: typechecker,
        }
        .into_symbol();
        typechecker.mut_current_scope().push_symbol(symbol.clone());
        Self { symbol, expression }
    }

    pub fn get_declaration(
        &self,
    ) -> Option<&parm_ast::parser::nodes::declaration::variable::LetStatement<'a>> {
        let inner = self.symbol.inner.borrow();
        let SymbolDeclaration::LetStatement(stmt) = inner.declaration else {
            return None;
        };
        Some(stmt)
    }
}
