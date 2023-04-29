use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use crate::{
    tokens::{Parse, Span, Token__},
    Token, Tokens,
};
#[derive(Clone, PartialEq, PartialOrd)]
pub struct Literal {
    /// discriminant
    pub span: Span,
    pub(crate) inner: implementation::Inner,
}
impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
impl Parse for Literal {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>> {
        let token = input.advance();
        if let Some(Token::Lit(token)) = token {
            Ok(token.clone())
        } else {
            Err(format!("Expected {}, got {:?}", stringify!(Literal), token).into())
        }
    }
}
impl Literal {
    pub fn new_str(span: Span, str: String) -> Self {
        Literal {
            span,
            inner: implementation::Inner::String(str),
        }
    }
    pub fn new_num(span: Span, num: f64) -> Self {
        Literal {
            span,
            inner: implementation::Inner::Number(num),
        }
    }
    pub fn new_bool(span: Span, bool: bool) -> Self {
        Literal {
            span,
            inner: implementation::Inner::Boolean(bool),
        }
    }
}
impl Token__ for Literal {
    fn span(&self) -> Span {
        self.span
    }
    fn display(&self) -> String
    where
        Self: Sized,
    {
        format!("{}", self)
    }
}
impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            implementation::Inner::String(s) => write!(f, "{}", s),
            implementation::Inner::Number(n) => write!(f, "{}", n),
            implementation::Inner::Boolean(b) => write!(f, "{}", b),
        }
    }
}
mod implementation {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub(crate) enum Inner {
        String(String),
        Number(f64),
        Boolean(bool),
    }
}
