use std::{error::Error, ops::Range};

use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;

use colored::*;
#[derive(Debug)]
pub enum ParseError {
    ExpectedToken(Token, Token),
    Generic(String),
    Boxed(Box<dyn Error>),
    ExpectedEnd(Token),
    EndOfTokens,
}
mod n {
    use std::any::{self, Any};

    use super::*;
    #[derive(Debug, Clone)]
    pub struct ParseError {
        pub surrounding: TokenStream,
        pub kind: ParseErrorKind,
    }
    #[derive(Debug, Clone)]
    pub enum ParseErrorKind {
        ExpectedToken(ExpectedToken),
        ExpectedEnd(Token),
        ExpectedAst(ExpectedAst),
    }

    #[derive(Debug, Clone)]
    pub struct ExpectedAst {
        ast: any::TypeId,
    }
    #[derive(Debug, Clone)]
    pub struct ExpectedToken {
        expected: Token,
        at: usize, // <- an index to `surrounding`
    }
}
#[derive(Debug, Clone)]
pub struct TokenErrorInfo {
    tokens: Vec<Token>,
    idxs: Vec<(Range<usize>, String)>,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EndOfTokens => {
                write!(f, "{}", "Error: ".bold().red())?;
                write!(f, "Unexpected end of tokens")
            }
            ParseError::Generic(s) => {
                write!(f, "{}", "Error: ".bold().red())?;
                write!(f, "{}", s)
            }
            ParseError::Boxed(e) => {
                write!(f, "{}", "Error: ".bold().red())?;
                write!(f, "{}", e)
            }
            ParseError::ExpectedToken(expected, got) => {
                write!(f, "{}", "Error: ".bold().red())?;
                write!(
                    f,
                    "Expected token `{}`, got {}",
                    expected.to_string().bold(),
                    match got {
                        Token::Ident(ident) => format!("identifier `{}`", ident.to_string().bold()),
                        _ => format!("token `{}`", got).bold().to_string(),
                    }
                )
            }
            ParseError::ExpectedEnd(Token) => {
                write!(f, "{}", "Error: ".bold().red())?;
                write!(f, "Expected end of file, got {:?}", Token)
            }
        }
    }
}
impl Error for ParseError {}
