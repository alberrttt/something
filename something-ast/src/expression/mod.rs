use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{ident::Ident, lit::Literal, tokens::Token, Parse, Tokens};
pub mod precedence;
#[derive(Debug, Clone)]
pub enum Expression {
    Lit(Literal),
    Binary(Binary),
}

impl Parse for Expression {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let tmp = input.peek().unwrap().clone();
        parse_expr(
            match tmp {
                Token::Lit(lit) => {
                    input.advance().unwrap();
                    Self::Lit(lit)
                }
                x => panic!("{:?}", x),
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
        Token::Plus(_) => Ok(parse_binary(left, input)),
        _ => Ok(left),
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
fn parse_binary(left: Expression, input: &mut Tokens) -> Expression {
    let operator = Operator::parse(input).unwrap();
    let right = Expression::parse(input).unwrap();
    Expression::Binary(Binary {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    })
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
