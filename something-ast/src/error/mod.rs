use colored::Colorize;

use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub surrounding: TokenStream,
    pub kind: ParseErrorKind,
}
#[allow(non_snake_case)]
impl ParseError {
    pub fn ParseFloatError(err: std::num::ParseFloatError) -> Self {
        Self::create(
            &Token::Eof(Default::default()),
            ParseErrorKind::FloatParseError(err),
        )
    }
    pub fn ExpectedEnd(token: Token) -> Self {
        Self::create(&token, ParseErrorKind::ExpectedEnd(token.clone()))
    }
    pub fn EndOfTokens(backtrace: Backtrace) -> Self {
        Self::create(&Token::Eof(Default::default()), ParseErrorKind::EndOfTokens)
    }
    pub fn Generic(msg: String) -> Self {
        Self::create(
            &Token::Eof(Default::default()),
            ParseErrorKind::Generic(msg),
        )
    }
    pub fn ExpectedToken(expected: Token, got: Token) -> Self {
        Self::create(
            &got,
            ParseErrorKind::ExpectedToken(ExpectedToken { expected, at: 0 }),
        )
    }
}
impl ParseError {
    fn create(surrounding_tokens: &dyn AppendTokens, kind: ParseErrorKind) -> Self {
        let mut tokenstream = TokenStream::default();
        surrounding_tokens.append_tokens(&mut tokenstream);
        Self {
            surrounding: tokenstream,
            kind,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    ExpectedToken(ExpectedToken),
    ExpectedEnd(Token),
    ExpectedAst(ExpectedAst),
    EndOfTokens,
    Generic(String),
    FloatParseError(std::num::ParseFloatError),
}
use std::backtrace::Backtrace;
use std::error::Error;
use std::{any, default};
#[derive(Debug, Clone)]
pub struct ExpectedAst {
    ast: any::TypeId,
}
#[derive(Debug, Clone)]
pub struct ExpectedToken {
    expected: Token,
    at: usize, // <- an index to `surrounding`
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Error ".red().bold())?;
        use ParseErrorKind::*;
        match &self.kind {
            FloatParseError(float_parse_error) => {
                write!(f, "{}", float_parse_error)
            }
            ExpectedToken(token) => {
                write!(
                    f,
                    "Expected token {:?} instead of {}",
                    token.expected,
                    self.surrounding.get(token.at).unwrap()
                )
            }
            ExpectedEnd(_) => todo!(),
            ExpectedAst(_) => todo!(),
            EndOfTokens => {
                write!(f, "End of token stream")
            }
            Generic(string) => {
                write!(f, "{}", string)
            }
        }
    }
}
impl std::error::Error for ParseError {}
