pub mod printer;
use std::{
    backtrace::Backtrace, cmp::Ordering, collections::HashMap, error::Error, fmt::Display, slice,
    vec,
};

use parm_common::{Span, Spanned};

use crate::lexer::{
    token::{tokens_by_line, Token},
    Lexer,
};

use std::fmt::Write;
fn display_tokens_with_annotations<'a>(
    tokens: &[Token<'a>],
    annotations: HashMap<usize, String>,
) -> Result<String, Box<dyn Error>> {
    let mut f = String::new();
    let lines = tokens_by_line(tokens);
    let mut idx = 0;
    let mut line_offset = 0;
    let mut annotation_location = HashMap::new();

    for (line, tokens) in lines.iter().enumerate() {
        let mut prev_token: *const Token<'_> = std::ptr::null();

        for (line_idx, token) in tokens.iter().enumerate() {
            if line_idx == 0 {
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
            prev_token = token;
            if let Some(annotation) = annotations.get(&idx) {
                annotation_location.insert(line_offset, idx);
            }
            idx += 1;
            line_offset += 1;
        }

        if line != lines.len() - 1 {
            writeln!(f)?;
        }

        if !annotation_location.is_empty() {
            for (line_offset, annotation) in annotation_location.iter() {
                write!(
                    f,
                    "{} {:whitespace$}",
                    "^".repeat(tokens[*annotation].lexeme().len()),
                    annotations.get(annotation).unwrap(),
                    whitespace = line_offset
                )?;
            }
            writeln!(f)?;
        }

        line_offset = 0;
        annotation_location.clear();
    }
    Ok(f)
}
fn display_tokens<'a>(tokens: &[Token<'a>]) -> Result<String, Box<dyn Error>> {
    let mut f = String::new();
    let lines = tokens_by_line(tokens);

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
    Ok(f)
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

    pub fn print(&self) -> String {
        let mut result = String::new();
        let displayed = display_tokens_with_annotations(self.surrounding, {
            let mut map = HashMap::new();
            map.insert(0, "error".to_string());
            map
        });
        writeln!(result, "{}", displayed.unwrap()).unwrap();

        match self.kind {
            ErrorKind::EndOfTokens(_) => {
                write!(result, "Unexpected end of tokens").unwrap();
            }
            ErrorKind::ExpectedToken(_) => todo!(),
            ErrorKind::ExpectedNode(_) => todo!(),
        }
        result
    }
}
impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // check if PDB is set
        // Print Diagnostic Backtrace
        if std::env::var("PDB").is_ok() {
            write!(f, "{}", self.backtrace.as_ref().unwrap())?;
        }

        write!(f, "{}", self.print())
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
        match self {
            ErrorKind::EndOfTokens(e) => write!(f, "{}", e),
            ErrorKind::ExpectedToken(e) => write!(f, "{}", e),
            ErrorKind::ExpectedNode(e) => write!(f, "{}", e),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EndOfTokens {}
impl Display for EndOfTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, Default)]

pub struct ExpectedToken<'a> {
    pub expected: Token<'a>,
    pub got: Token<'a>,
}
impl<'a> Display for ExpectedToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExpectedNode<'a> {
    pub expected: &'static str,
    pub got: &'a str,
}

impl<'a> Display for ExpectedNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
