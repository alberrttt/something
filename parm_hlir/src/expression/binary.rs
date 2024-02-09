use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression<'a, 'b> {
    pub left: Box<Expression<'a, 'b>>,
    pub operator: &'b str,
    pub right: Box<Expression<'a, 'b>>,
}

