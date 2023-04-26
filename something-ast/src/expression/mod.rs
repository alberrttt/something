use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{ident::Ident, tokens::Token, Parse, Tokens};
#[derive(Debug, Clone, ParseTokens)]
pub enum Expression {
    Ident(Ident),

    Binary(Binary),
}
#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}
impl Parse for Binary {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            left: Box::new(Expression::parse(input)?),
            operator: Operator::parse(input)?,
            right: Box::new(Expression::parse(input)?),
        })
    }
}
#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
}
impl Parse for Operator {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        match input.advance() {
            Some(token) => Ok(match token {
                Token::Plus(_) => Self::Plus,
                Token::Minus(_) => Self::Minus,
                Token::Star(_) => Self::Multiply,
                Token::Slash(_) => Self::Divide,
                Token::Equal(_) => Self::Equal,
                _ => {
                    return Err(format!("Expected Operator, got {:?}", token.clone()).into());
                }
            }),
            _ => Err(format!("Expected Operator, got {:?}", input.advance().clone()).into()),
        }
    }
}
