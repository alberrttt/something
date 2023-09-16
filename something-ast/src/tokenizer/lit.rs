use super::prelude::*;
use crate::prelude::*;
use std::fmt::{Display, Formatter};
#[derive(Clone, PartialEq, PartialOrd, Eq, Default)]
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

impl Node for Literal {
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        let token = parser.advance()?;
        if let Token::Lit(token) = token {
            Ok(token.clone())
        } else {
            Err(ParseError::expected_token(
                Token::Lit(Literal::default()),
                token.clone(),
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
            inner: lit_impl::Inner::Float(num),
        }
    }
    pub fn new_bool(span: Span, bool: bool) -> Self {
        Literal {
            span,
            inner: lit_impl::Inner::Boolean(bool),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            lit_impl::Inner::String(s) => write!(f, "{}", s),
            lit_impl::Inner::Float(n) => write!(f, "{}", n),
            lit_impl::Inner::Boolean(b) => write!(f, "{}", b),
            lit_impl::Inner::Integer(i) => write!(f, "{}", i),
        }
    }
}
pub mod lit_impl {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub enum Inner {
        String(String),
        Float(f64),
        Integer(i64),
        Boolean(bool),
    }
    impl Default for Inner {
        fn default() -> Self {
            Self::String(String::new())
        }
    }
}
impl Eq for lit_impl::Inner {
    fn assert_receiver_is_total_eq(&self) {}
}
