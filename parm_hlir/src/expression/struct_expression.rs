use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use parm_ast::parser::nodes::path::SimpleSegment;
use parm_common::Spanned;

use crate::{
    error::TypeError,
    symbol::{self, InnerSymbol, Symbol, SymbolDeclaration},
    traits::{Check, TypeCheckResult},
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
impl<'a, 'b> Check<'a, 'b, StructExpression<'a, 'b>>
    for parm_ast::parser::nodes::expression::expr_struct::StructExpression<'a>
{
    fn check(
        &'b self,
        tc: &mut crate::typechecker::Typechecker<'a, 'b>,
    ) -> TypeCheckResult<'a, 'b, StructExpression<'a, 'b>> {
        let name = &self.name;
        let name = name.first_segment();
        let SimpleSegment::Identifier(name) = name else {
            panic!()
        };
        let symbol = tc.get_symbol(name.lexeme).unwrap();
        let Type::Struct(ref struct_ty) = &symbol.inner.as_ref().borrow().ty else {
            panic!("Struct {} not found", name.lexeme);
        };
        let mut members = vec![];
        for field in self.body.elements() {
            let name = &field.ident;
            let ast_expr = &field.expr;

            let field = struct_ty
                .fields
                .iter()
                .find(|f| f.inner.borrow().lexeme == name.lexeme)
                .unwrap();
            let expr = ast_expr.check(tc)?;
            let field_ty = &field.inner.borrow().ty;
            let expr_ty = expr.get_ty();
            if !field_ty.eq_amb(&expr_ty) {
                tc.errs.push(TypeError::new(
                    crate::error::TypeErrorKind::MismatchedTypes {
                        expected: field_ty.clone(),
                        got: expr_ty.clone(),
                        location: ast_expr.span(),
                    },
                    tc.source_file.preparsed,
                ));
            }
            members.push(StructMemberInitialization {
                symbol: field.clone(),
                expression: expr,
            });
        }
        Ok(StructExpression {
            symbol: symbol.clone(),
            members,
            ast: AST(self),
        })
    }
}
