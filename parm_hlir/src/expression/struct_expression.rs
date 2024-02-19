use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{
    symbol::{self, InnerSymbol, Symbol, SymbolDeclaration},
    traits::Check,
    ty::Type,
    AST,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StructExpression<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub members: Vec<StructMemberInitialization<'a, 'b>>,
    pub ast: AST<&'b parm_ast::parser::nodes::expression::expr_struct::StructExpression<'a>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StructMemberInitialization<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub expression: crate::expression::Expression<'a, 'b>,
}
impl<'a, 'b> Check<'a, 'b> for StructExpression<'a, 'b> {
    type Output = Self;

    type Ast = parm_ast::parser::nodes::expression::expr_struct::StructExpression<'a>;

    fn check(tc: &mut crate::typechecker::Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output {
        let symbol = tc.get_symbol(ast.ident.lexeme).unwrap();
        let Type::Struct(ref struct_ty) = &symbol.inner.as_ref().borrow().ty else {
            panic!("Struct {} not found", ast.ident.lexeme);
        };
        let mut members = vec![];
        for field in ast.body.elements() {
            let name = &field.ident;
            let expr = &field.expr;

            let field = struct_ty
                .fields
                .iter()
                .find(|f| f.inner.borrow().lexeme == name.lexeme)
                .unwrap();
            let expr = crate::expression::Expression::check(tc, expr);
            if !expr.get_ty().eq_amb(&field.inner.borrow().ty) {
                panic!(
                    "Type mismatch {:?} != {:?}",
                    expr.get_ty(),
                    field.inner.borrow().ty
                );
            }
            members.push(StructMemberInitialization {
                symbol: field.clone(),
                expression: expr,
            });
        }
        Self {
            symbol: symbol.clone(),
            members,
            ast: AST(ast),
        }
    }
}
