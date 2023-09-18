use std::backtrace::Backtrace;
use std::f32::consts::E;
use std::fmt::Display;
use std::rc::Rc;

use colored::Colorize;
use log::{BodyLine, Header, Log, LogBody};

use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;
#[derive(Debug, Clone, Default)]
pub struct ParseError {
    pub surrounding: Option<TokenStream>,
    pub kind: ParseErrorKind,
    pub backtrace: Option<Rc<Backtrace>>,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.backtrace {
            None => {}
            Some(backtrace) => {
                println!("{backtrace}");
            }
        }
        match &self.kind {
            ParseErrorKind::EndOfTokens => {
                let log = Log {
                    header: Header::default().err().push("end of tokens"),
                    body: LogBody::default()
                        .push("expected more tokens")
                        .push("reached end of tokens"),
                };
                write!(f, "{}", log)
            }
            ParseErrorKind::ExpectedToken(expected, got) => {
                dbg!(expected);
                let log = Log {
                    header: Header::default().err().push("expected token"),
                    body: LogBody::default()
                        .push(format!("expected: {}", expected).as_ref())
                        .push(format!("got: {}", got).red().bold()),
                };
                write!(f, "{}", log)
            }
            ParseErrorKind::InRecovery => todo!(),
        }
    }
}
impl ParseError {
    pub fn expected_token(expected: Token, got: Token) -> Self {
        Self {
            surrounding: None,
            kind: ParseErrorKind::ExpectedToken(expected, got),
            backtrace: Some(Rc::new(Backtrace::capture())),
        }
    }
    pub fn end_of_tokens() -> Self {
        Self {
            surrounding: None,
            kind: ParseErrorKind::EndOfTokens,
            backtrace: Some(Rc::new(Backtrace::capture())),
        }
    }
    pub fn expected_token_stream(expected: TokenStream, got: TokenStream) -> Self {
        todo!()
    }
}
#[derive(Debug, Clone, Default)]
pub enum ParseErrorKind {
    ExpectedToken(Token, Token),
    EndOfTokens,
    #[default]
    InRecovery,
}
