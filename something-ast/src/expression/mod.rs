use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{
    delimiter::Delimiter, ident::Ident, lit::Literal, tokens::Token, Parse, Tokens,
};
pub mod precedence;
#[derive(Debug, Clone)]
pub enum Expression {
    Lit(Literal),
    Binary(Binary),
    Call(Call),
    Grouping(Parenthesis<Box<Expression>>),
}
mod call;
use crate::delimiter::Parenthesis;

pub use self::call::*;
impl Parse for Expression {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let tmp = match input.peek() {
            Some(token) => token.clone(),
            None => {
                return Err(format!("end of file").into());
            }
        };
        parse_expr(
            match tmp {
                Token::Lit(lit) => {
                    input.advance();
                    Self::Lit(lit)
                }
                Token::Ident(ident) => Expression::Call(Call::parse(input)?),
                x => panic!("Expected a token to start an expression, but got {:?}", x),
            },
            input,
        )
    }
}
fn parse_expr(
    left: Expression,
    input: &mut Tokens,
) -> Result<Expression, Box<dyn std::error::Error>> {
    let token = match input.peek() {
        Some(token) => token.clone(),
        _ => {
            return Ok(left);
        }
    };
    match token {
        Token::Plus(_) | Token::Minus(_) => match Operator::parse(input) {
            Ok(operator) => {
                let right = Expression::parse(input).expect("Expected Expression");
                Ok(Expression::Binary(Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }))
            }

            Err(_) => Ok(left),
        },
        Token::Star(_) | Token::Slash(_) => match Operator::parse(input) {
            Ok(operator) => {
                let right = Literal::parse(input).expect("Expected Expression");
                parse_expr(
                    Expression::Binary(Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(Expression::Lit(right)),
                    }),
                    input,
                )
            }

            Err(_) => Ok(left),
        },

        token => Ok(left),
    }
}
#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}
impl From<Binary> for Expression {
    fn from(binary: Binary) -> Self {
        Self::Binary(binary)
    }
}

impl Parse for Binary {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let expr = Expression::parse(input)?;
        todo!();
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
