use parm_common::Spanned;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Number<'a> {
    pub token: &'a Token<'a>,
    pub value: f64,
}
impl<'a> TreeDisplay for Number<'a> {
    fn tree(&self) -> Tree {
        Tree::new("Number").lexeme(format!("{}", self.value))
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
    fn parse(parse_stream: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let num = parse_stream.peek()?;
        match num {
            Token::Integer(int) => {
                parse_stream.advance()?;
                Ok(Number {
                    token: num,
                    value: int.lexeme.parse::<f64>().unwrap(),
                })
            }
            Token::Float(float) => {
                parse_stream.advance()?;
                Ok(Number {
                    token: num,
                    value: float.lexeme.parse::<f64>().unwrap(),
                })
            }
            token => {
                dbg!(token);
                ParseError::err(
                    crate::error::ErrorKind::ExpectedToken(ExpectedToken {
                        expected: Token::Integer(Integer::default()),
                        got: token.clone(),
                        location: parse_stream.current,
                    }),
                    parse_stream.tokens,
                    parse_stream.src_file,
                )
            }
        }
    }
}
