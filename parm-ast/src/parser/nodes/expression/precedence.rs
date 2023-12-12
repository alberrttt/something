use crate::prelude::Token;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Grouping,
}

impl Precedence {
    pub fn value(&self) -> u8 {
        *self as u8
    }
    pub fn increment(&self) -> Self {
        use Precedence::*;
        match self {
            None => Assignment,
            Assignment => Equality,
            Equality => Comparison,
            Comparison => Term,
            Term => Factor,
            Factor => Unary,
            Unary => Grouping,
            Grouping => None,
        }
    }
}

impl<'a> From<&'a Token<'a>> for Precedence {
    fn from(value: &'a Token<'a>) -> Self {
        match value {
            Token::Plus(_) | Token::Minus(_) => Precedence::Term,
            Token::Asterisk(_) | Token::Slash(_) | Token::AsteriskAsterisk(_) => Precedence::Factor,
            _ => Precedence::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorInfo {
    Prefix,
    Suffix,
}
