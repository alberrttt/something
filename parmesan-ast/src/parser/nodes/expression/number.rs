use parmesan_common::Spanned;

use crate::lexer::token::Token;

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
