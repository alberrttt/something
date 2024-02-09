use crate::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a, 'b> {
    Expression(Expression<'a, 'b>),
}
