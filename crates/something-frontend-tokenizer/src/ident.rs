use crate::traits::ParsingDisplay;
use crate::Parse;
use crate::{tokens::Span, Token, Tokens};
use std::fmt::Debug;
use std::{error::Error, fmt::Display};

#[derive(Clone, Eq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}
impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl std::hash::Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ident<{}>", self.name)
    }
}
impl ParsingDisplay for Ident {
    fn display(&self) -> String
    where
        Self: Sized,
    {
        self.name.clone()
    }

    fn placeholder() -> String
    where
        Self: Sized,
    {
        "<identifier>".into()
    }
}
impl Parse for Ident {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>> {
        let token = input.advance();
        if let Some(Token::Ident(token)) = token {
            Ok(token.clone())
        } else {
            Err(format!("Expected Ident, got {:?}", token).into())
        }
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
