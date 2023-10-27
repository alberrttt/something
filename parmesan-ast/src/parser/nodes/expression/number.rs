use parmesan_common::Spanned;

use crate::{
    error::ExpectedToken,
    lexer::token::{Integer, Token},
    traits::Node,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Number<'a> {
    token: &'a Token<'a>,
    pub value: f64,
}
impl<'a> From<&'a Token<'a>> for Number<'a> {
    fn from(token: &'a Token<'a>) -> Self {
        Number {
            token,
            value: match token {
                Token::Float(float) => float.lexeme,
                Token::Integer(int) => int.lexeme,
                _ => panic!(),
            }
            .parse::<f64>()
            .unwrap(),
        }
    }
}
impl Spanned for Number<'_> {
    fn span(&self) -> parmesan_common::Span {
        self.token.span()
    }
}
impl<'a> Node<'a> for Number<'a> {
    fn parse(parser: &mut crate::parser::Parser<'a>) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let num = parser.peek()?;
        match num {
            Token::Integer(int) => {
                parser.advance()?;
                Ok(Number {
                    token: num,
                    value: int.lexeme.parse::<f64>().unwrap(),
                })
            }
            Token::Float(float) => {
                parser.advance()?;
                Ok(Number {
                    token: num,
                    value: float.lexeme.parse::<f64>().unwrap(),
                })
            }
            token => Err(crate::error::ParseError::ExpectedToken(ExpectedToken {
                expected: Token::Integer(Integer::default()),
                got: token.clone(),
            })),
        }
    }
}
