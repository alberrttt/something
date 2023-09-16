use super::prelude::*;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Default, PartialOrd, Ord, Debug)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Node for Ident {
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        let token = parser.advance()?;
        if let Token::Ident(token) = token {
            Ok(token.clone())
        } else {
            Err(ParseError::expected_token(
                Token::Ident(Ident::default()),
                token.clone(),
                // unwrap CAN panic, since it might be EOF, so there arent any more tokens
            ))
        }
    }
    fn span(&self) -> Span {
        self.span
    }

    fn append_tokens(&self, to: &mut Vec<Token>) {
        todo!()
    }
}
impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
impl From<&str> for Ident {
    fn from(value: &str) -> Self {
        Self {
            name: value.into(),
            span: Span::default(),
        }
    }
}
