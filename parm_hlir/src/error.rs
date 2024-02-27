use std::{
    collections::HashMap,
    fmt::{format, Error},
};

use parm_ast::{
    error::{display_annotations, Annotation},
    lexer::token::Token,
    source_file::PreparsedSourceFile,
};
use parm_common::{Span, Spanned};

use crate::prelude::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeError<'a, 'b: 'a> {
    pub kind: TypeErrorKind<'a, 'b>,
    pub file: &'b PreparsedSourceFile<'a>,
}
impl<'a, 'b: 'a> TypeError<'a, 'b> {
    pub fn new(kind: TypeErrorKind<'a, 'b>, file: &'b PreparsedSourceFile<'a>) -> Self {
        Self { kind, file }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeErrorKind<'a, 'b: 'a> {
    MismatchedTypes {
        expected: Type<'a, 'b>,
        got: Type<'a, 'b>,
        location: Span,
    },
    TypeNameNotFound {
        name: &'b str,
        location: Span,
    },
    SymbolNotFound {
        name: &'b str,
        location: Span,
    },
    NotCallable {
        location: Span,
    },
    IncorrectArgs {
        expected: u8,
        got: u8,
        location: Span,
    },
}
impl<'a, 'b: 'a> std::fmt::Display for TypeError<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = HashMap::new();
        match &self.kind {
            TypeErrorKind::NotCallable { location } => {
                map.insert(
                    *location,
                    Annotation::new("Expression is not callable").auto(),
                );
            }
            TypeErrorKind::MismatchedTypes {
                expected,
                got,
                location,
            } => {
                map.insert(
                    *location,
                    Annotation::new(format!("Type Mismatch: expected {}, got {}", expected, got,))
                        .auto(),
                );
            }
            TypeErrorKind::TypeNameNotFound { name, location } => {
                map.insert(
                    *location,
                    Annotation::new(format!("Type name `{}` not found", name)).auto(),
                );
            }
            TypeErrorKind::SymbolNotFound { name, location } => {
                map.insert(
                    *location,
                    Annotation::new(format!("Symbol `{}` not found", name)).auto(),
                );
            }
            TypeErrorKind::IncorrectArgs {
                expected,
                got,
                location,
            } => {
                map.insert(
                    *location,
                    Annotation::new(format!(
                        "Incorrect amount of argument(s): expected {}, got {}",
                        expected, got
                    ))
                    .auto(),
                );
            }
        }
        let Ok((result, token)) = display_annotations(self.file, map) else {
            return Err(Error);
        };
        let span = token.span();
        writeln!(
            f,
            "{}:{}:{}",
            self.file.path.to_str().unwrap(),
            span.line,
            span.line_start
        )?;
        write!(f, "{}", result)
    }
}