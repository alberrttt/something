use parmesan_common::{Span, Spanned};

use crate::lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier<'a> {
    token: &'a Token<'a>,
}
impl<'a> From<&'a Token<'a>> for Identifier<'a> {
    fn from(token: &'a Token<'a>) -> Self {
        assert!(matches!(token, Token::Ident(_)));
        Identifier { token }
    }
}
impl<'a> Identifier<'a> {
    pub fn text(&self) -> &'a str {
        self.token.lexeme()
    }
}

impl Spanned for Identifier<'_> {
    fn span(&self) -> Span {
        self.token.span()
    }
}
