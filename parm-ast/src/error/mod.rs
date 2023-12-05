pub mod printer;
use std::{backtrace::Backtrace, cmp::Ordering, error::Error, fmt::Display, slice, vec};

use parm_common::{Span, Spanned};

use crate::lexer::{
    token::{tokens_by_line, Token},
    Lexer,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ErrorMessage<'a> {
    tokens: Vec<Token<'a>>,
    messages: Vec<(Span, String)>,
}

impl<'a> Display for ErrorMessage<'a> {
    /// note: this might be unsafe if the tokens aren't in correct order.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = tokens_by_line(&self.tokens);

        for (i, line) in lines.iter().enumerate() {
            let mut prev_token: *const Token<'_> = std::ptr::null();

            for (i, token) in line.iter().enumerate() {
                if i == 0 {
                    write!(
                        f,
                        "{:whitespace$}",
                        "",
                        whitespace = token.span().line_start
                    )?;
                } else {
                    let prev_token = unsafe { &*prev_token };
                    write!(
                        f,
                        "{:whitespace$}",
                        "",
                        // whitespace = current token start - previous token end
                        whitespace = token.span().line_start
                            - (prev_token.span().line_start + prev_token.span().src_end
                                - prev_token.span().src_start)
                    )?;
                }
                write!(f, "{}", token.lexeme())?;
                prev_token = token
            }
            if i != lines.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
#[test]
fn test_error_message() {
    // let mut lexer = Lexer::from("line1 item2\nline2\nline3");
    // let tokens = lexer.lex();
    // let msg = ErrorMessage {
    //     tokens,
    //     messages: vec![],
    // };

    // assert_eq!(msg.to_string(), "line1 item2\nline2\nline3")
}
#[derive(Debug)]
pub struct ParseError<'a> {
    pub kind: ErrorKind<'a>,
    pub backtrace: Option<Backtrace>,
    pub surrounding: &'a [Token<'a>],
}
impl<'a> Error for ParseError<'a> {}
impl<'a> PartialEq for ParseError<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.surrounding == other.surrounding
    }
}
impl<'a> Clone for ParseError<'a> {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            backtrace: self.backtrace.as_ref().map(|_| Backtrace::capture()),
            surrounding: self.surrounding,
        }
    }
}
impl<'a> ParseError<'a> {
    pub fn new(kind: ErrorKind<'a>, surrounding: &'a [Token<'a>]) -> Self {
        Self {
            kind,
            surrounding,
            backtrace: Some(Backtrace::capture()),
        }
    }
}
impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // check if PEB is set
        if std::env::var("PEB").is_ok() {
            write!(f, "{}", self.backtrace.as_ref().unwrap())?;
        }

        write!(f, "{}", self.kind)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind<'a> {
    EndOfTokens(EndOfTokens),
    ExpectedToken(ExpectedToken<'a>),
    ExpectedNode(ExpectedNode<'a>),
}
impl Error for ErrorKind<'_> {}
impl Display for ErrorKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EndOfTokens {}
#[derive(Debug, Clone, PartialEq, Default)]

pub struct ExpectedToken<'a> {
    pub expected: Token<'a>,
    pub got: Token<'a>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExpectedNode<'a> {
    pub expected: &'static str,
    pub got: &'a str,
}
