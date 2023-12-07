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
#[derive(Debug, PartialEq, Clone)]
pub struct Annotation {
    pub message: String,
    pub offset: usize,
    pub after: bool,
    // size of the arrows; usize::MAX: automatically calculate size, based on the length of the token
    pub size: usize,
}
impl Annotation {
    pub fn new(message: String) -> Self {
        Annotation {
            message,
            offset: 0,
            after: false,
            size: usize::MAX,
        }
    }

    pub fn after(mut self) -> Self {
        self.after = true;
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn auto(mut self) -> Self {
        self.size = usize::MAX;
        self
    }
}

fn display_tokens_with_annotations<'a>(
    tokens: &[Token<'a>],
    annotations: HashMap<usize, Annotation>,
) -> Result<String, Box<dyn Error>> {
    let mut f = String::new();
    let lines = tokens_by_line(tokens);
    let mut idx = 0;
    let mut annotation_location = HashMap::new();

    for (line, token_on_line) in lines.iter().enumerate() {
        let mut prev_token: *const Token<'_> = std::ptr::null();

        for (line_idx, token) in token_on_line.iter().enumerate() {
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
                annotation_location.insert(token.span().line_start, idx);
            }
            idx += 1;
        }

        writeln!(f)?;

        if !annotation_location.is_empty() {
            for (line_offset, annotation_idx) in annotation_location.iter() {
                let annotation = annotations.get(annotation_idx).unwrap();
                let token = &tokens[*annotation_idx];
                let len = token.lexeme().len();
                write!(
                    f,
                    "{:whitespace$}{} {}",
                    "",
                    "^".repeat(if annotation.size == usize::MAX {
                        len
                    } else {
                        annotation.size
                    }),
                    annotation.message,
                    whitespace =
                        *line_offset + annotation.offset + if annotation.after { len } else { 0 },
                )?;
            }
            writeln!(f)?;
        }

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
    #[track_caller]
    pub fn new(kind: ErrorKind<'a>, surrounding: &'a [Token<'a>]) -> Self {
        Self {
            kind,
            surrounding,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn print(&self) -> String {
        let mut result = String::new();
        let mut map: HashMap<usize, Annotation> = HashMap::new();
        match &self.kind {
            ErrorKind::EndOfTokens(eot) => {
                map.insert(self.surrounding.len() - 1, {
                    let mut annotation = String::from("unexpected token end");
                    if let Some(expected) = eot.expected {
                        write!(annotation, ", expected a(n) {}", expected).unwrap();
                    }
                    Annotation::new(annotation).after().size(1)
                });
            }
            ErrorKind::ExpectedToken(expected) => {
                let mut coresponing_token = 0usize;
                for (i, token) in self.surrounding.iter().enumerate() {
                    if token.span() == expected.expected.span() {
                        coresponing_token = i;
                        break;
                    }
                }
                map.insert(
                    coresponing_token,
                    Annotation::new(format!(
                        "Expected {} but got {}",
                        expected.expected.lexeme(),
                        expected.got.lexeme()
                    )),
                );
            }
            ErrorKind::ExpectedNode(expected) => {
                let mut coresponing_token = 0usize;
                for (i, token) in self.surrounding.iter().enumerate() {
                    if token.span().src_start == expected.got.as_bytes().as_ptr() as usize {
                        coresponing_token = i;
                        break;
                    }
                }
                map.insert(
                    coresponing_token,
                    Annotation::new(format!(
                        "Expected {} but got {}",
                        expected.expected, expected.got
                    )),
                );
            }
        }

        let displayed = display_tokens_with_annotations(self.surrounding, map);
        writeln!(result, "{}", displayed.unwrap()).unwrap();
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
    EndOfTokens(EndOfTokens<'a>),
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
pub struct EndOfTokens<'a> {
    pub expected: Option<&'a str>,
}
impl Display for EndOfTokens<'_> {
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
