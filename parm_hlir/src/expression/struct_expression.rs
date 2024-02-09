use crate::symbol::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub struct StructExpression<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub ast: &'b parm_ast::parser::nodes::expression::expr_struct::StructExpression<'a>,
    
}
