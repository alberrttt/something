use parm_common::Spanned;

use crate::{
    error::ExpectedToken,
    lexer::token::{Integer, Token},
    parser::ast_displayer::DisplayNode,
    prelude::{ParseError, ParseResult},
    traits::{CreateDisplayNode, Node},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Number<'a> {
    token: &'a Token<'a>,
    pub value: f64,
}
impl CreateDisplayNode for Number<'_> {
    fn create_display_node(&self) -> crate::parser::ast_displayer::DisplayNode {
        crate::parser::ast_displayer::DisplayNode::new(self.value.to_string())
    }
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
    fn span(&self) -> parm_common::Span {
        self.token.span()
    }
}
impl<'a> Node<'a> for Number<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
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
            token => {
                dbg!(token);
                Err(ParseError::new(
                    crate::error::ErrorKind::ExpectedToken(ExpectedToken {
                        expected: Token::Integer(Integer::default()),
                        got: token.clone(),
                        location: parser.current,
                    }),
                    parser.tokens,
                ))
            }
        }
    }
}
