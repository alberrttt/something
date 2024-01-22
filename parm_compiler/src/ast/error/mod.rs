pub mod printer;
use std::{
    backtrace::Backtrace, collections::HashMap, error::Error, fmt::Display, ops::Range, vec,
};

use parm_common::{Span, Spanned};

use crate::{
    ast::lexer::token::{self, tokens_by_line, Token},
    ast::prelude::{ParseResult, PreparsedSourceFile},
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
    pub fn new(message: impl Into<String>) -> Self {
        Annotation {
            message: message.into(),
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

pub fn display_annotations<'a>(
    source: &'a PreparsedSourceFile<'a>,
    annotations: HashMap<Span, Annotation>,
) -> Result<(String, Token<'a>), Box<dyn Error>> {
    let mut f = String::new();
    let tokens = &source.lexer.tokens;
    let lines = tokens_by_line(tokens);
    let mut idx = 0;
    let mut annotation_location: HashMap<usize, Span> = HashMap::new();
    let mut used_lines = vec![];
    let mut significant_token_span = tokens.last().unwrap().clone();
    for (_, token_on_line) in lines.iter() {
        for _token in token_on_line.iter() {
            // we dont need to do this but im too lazyyt
            let line = _token.span().line;
            for annotation in annotations.keys() {
                if annotation.line == line && !used_lines.contains(&line) {
                    used_lines.push(line);
                }
            }
        }
    }
    let mut accounted_annotations = vec![];
    for (_line, token_on_line) in lines.iter() {
        let mut prev_token: Option<&Token> = None;
        if !used_lines.contains(&_line) {
            idx += token_on_line.len();
            continue;
        }
        let line: String = format!(
            "{:line$} | ",
            _line + 1,
            line = lines.len().to_string().len()
        );
        write!(f, "{line}")?;
        for (line_idx, token) in token_on_line.iter().enumerate() {
            if line_idx == 0 {
                write!(
                    f,
                    "{:whitespace$}",
                    "",
                    whitespace = token.span().line_start
                )?;
            } else {
                let prev_token = prev_token.unwrap();
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
            write!(
                f,
                "{}",
                if let Token::StringLiteral(str) = token {
                    format!("\"{}\"", token.lexeme())
                } else {
                    token.lexeme().to_string()
                }
            )?;
            prev_token = Some(token);

            for (i, annotation) in annotations.keys().enumerate() {
                let matches = token.span().src_start >= annotation.src_start
                    && annotation.line == token.span().line
                    && token.span().src_end <= annotation.src_end;
                if matches && !accounted_annotations.contains(&i) {
                    accounted_annotations.push(i);
                    annotation_location.insert(annotation.line_start, *annotation);
                    significant_token_span = token.clone();
                }
            }

            idx += 1;
        }

        writeln!(f)?;

        if !annotation_location.is_empty() {
            for (line_offset, annotation_idx) in annotation_location.iter() {
                let annotation = annotations.get(annotation_idx).unwrap();
                let len = annotation_idx.src_end - annotation_idx.src_start;
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
                    whitespace = *line_offset
                        + annotation.offset
                        + if annotation.after { len } else { 0 }
                        + line.len(),
                )?;
            }
            writeln!(f)?;
        }

        annotation_location.clear();
    }
    Ok((f, significant_token_span))
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
    pub file: &'a PreparsedSourceFile<'a>,
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
            file: self.file,
        }
    }
}
impl<'a> ParseError<'a> {
    #[track_caller]
    pub fn new(
        kind: ErrorKind<'a>,
        surrounding: &'a [Token<'a>],
        info: &'a PreparsedSourceFile<'a>,
    ) -> Self {
        Self {
            kind,
            surrounding,
            backtrace: Some(Backtrace::capture()),
            file: info,
        }
    }
    pub fn err<T>(
        kind: ErrorKind<'a>,
        surrounding: &'a [Token<'a>],
        info: &'a PreparsedSourceFile<'a>,
    ) -> ParseResult<'a, T> {
        Err(Box::new(Self::new(kind, surrounding, info)))
    }
}
impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // check if PDB is set
        // Display Diagnostic Backtrace
        if std::env::var("DDB").is_ok() {
            write!(f, "{}", self.backtrace.as_ref().unwrap())?;
        }

        let mut map: HashMap<Span, Annotation> = HashMap::new();
        match &self.kind {
            ErrorKind::EndOfTokens(eot) => {
                map.insert(self.surrounding.last().unwrap().span(), {
                    let mut annotation = String::from("unexpected end");
                    if let Some(expected) = eot.expected {
                        write!(annotation, ", expected `{}`", expected).unwrap();
                    }
                    Annotation::new(annotation).after().size(1)
                });
            }
            ErrorKind::ExpectedToken(expected) => {
                map.insert(
                    expected.got.span(),
                    Annotation::new(format!(
                        "Expected token {} but got {}",
                        expected.expected.lexeme(),
                        expected.got.lexeme()
                    )),
                );
            }
            ErrorKind::ExpectedNode(expected) => {
                map.insert(
                    expected.location,
                    Annotation::new(format!(
                        "Expected {} but got `{}`",
                        expected.expected, expected.got
                    )),
                );
            }
        }
        let (display, token) = display_annotations(self.file, map).unwrap();
        let span: Span = token.span();
        writeln!(
            f,
            "{}:{}:{}",
            self.file.path.to_str().unwrap(),
            span.line + 1,
            span.line_start + 1 + (span.src_end - span.src_start)
        )
        .unwrap();
        writeln!(f, "{display}",)
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
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, Default)]

pub struct ExpectedToken<'a> {
    pub expected: Token<'a>,
    pub got: Token<'a>,

    /// location in the source file's tokens
    pub location: usize,
}
impl<'a> Display for ExpectedToken<'a> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExpectedNode<'a> {
    pub expected: &'static str,
    pub got: &'a str,

    /// location in the source file's tokens
    pub location: Span,
}

impl<'a> Display for ExpectedNode<'a> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
