use crate::traits::ParsingDisplay;
use crate::Parse;
use crate::{
    tokens::{Span, Token__},
    Token, Tokens,
};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
#[derive(Clone, PartialEq, PartialOrd)]
pub struct Literal {
    /// discriminant
    pub span: Span,
    pub inner: lit_impl::Inner,
}
impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
impl ParsingDisplay for Literal {
    fn display(&self) -> String
    where
        Self: Sized,
    {
        use std::fmt::Write;
        let mut f = String::new();
        match &self.inner {
            lit_impl::Inner::String(s) => write!(f, "\"{}\"", s),
            lit_impl::Inner::Number(n) => write!(f, "{}", n),
            lit_impl::Inner::Boolean(b) => write!(f, "{}", b),
        };
        f
    }

    fn placeholder() -> String
    where
        Self: Sized,
    {
        "<literal>".into()
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
            inner: lit_impl::Inner::String(str),
        }
    }
    pub fn new_num(span: Span, num: f64) -> Self {
        Literal {
            span,
            inner: lit_impl::Inner::Number(num),
        }
    }
    pub fn new_bool(span: Span, bool: bool) -> Self {
        Literal {
            span,
            inner: lit_impl::Inner::Boolean(bool),
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
            lit_impl::Inner::String(s) => write!(f, "{}", s),
            lit_impl::Inner::Number(n) => write!(f, "{}", n),
            lit_impl::Inner::Boolean(b) => write!(f, "{}", b),
        }
    }
}
pub mod lit_impl {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub enum Inner {
        String(String),
        Number(f64),
        Boolean(bool),
    }
}
