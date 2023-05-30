use std::error::Error;

use crate::prelude::*;
use colored::*;
#[derive(Debug)]
pub enum ParseError {
    ExpectedToken(Token, Token),

    ExpectedEnd(Token),
    ExpectedAst(Vec<Box<dyn Name>>, Box<dyn Name>),
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ExpectedToken(expected, got) => {
                writeln!(f, "{}", "Error".bold().red())?;
                write!(f, "Expected token {:?}, got {:?}", expected, got)
            }
            ParseError::ExpectedEnd(Token) => {
                writeln!(f, "{}", "Error".bold().red())?;
                write!(f, "Expected end of file, got {:?}", Token)
            }
            ParseError::ExpectedAst(possibilities, got) => {
                if possibilities.len() > 1 {
                    writeln!(f, "{}", "Error".red().bold())?;
                    write!(f, "{}", "Expected a(n)".bold().red())?;
                    for (i, possibility) in possibilities.iter().enumerate() {
                        if i == possibilities.len() - 1 {
                            write!(f, "or {}", possibility.named())?;
                        } else {
                            write!(f, "{}, ", possibility.named())?;
                        }
                    }
                    write!(f, "\nGot: {}", got.named())
                } else {
                    writeln!(f, "{}", "Error".red().bold())?;
                    write!(f, "Expected {}", possibilities[0].named())?;
                    write!(f, "\nGot: {}", got.named())
                }
            }
        }
    }
}
impl Error for ParseError {}
