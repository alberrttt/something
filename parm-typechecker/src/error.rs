use parm_ast::{
    error::{display_annotations, Annotation},
    lexer::token::Token,
    parser::nodes::statement::{Span, Spanned},
    source_file::SourceFile,
};
use std::{backtrace::Backtrace, collections::HashMap, env, error::Error, fmt::Display};

use crate::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind<'a> {
    Mismatch(Mismatch<'a>),
    Incompatible(Incompatible<'a>),
    InvalidOperand(InvalidOperand<'a>),
}
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidOperand<'a> {
    pub operand: &'a str,
    pub location: Span,
    pub type1: &'a Type,
    pub type2: Option<&'a Type>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Incompatible<'a> {
    pub type1: &'a Type,
    pub type2: &'a Type,
    pub location: Span,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Mismatch<'a> {
    pub got: &'a Type,
    pub expected: &'a Type,
    pub location: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeError<'a> {
    pub kind: ErrorKind<'a>,
    pub file: &'a SourceFile<'a>,
}
impl<'a> Display for TypeError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut annotations: HashMap<Span, parm_ast::error::Annotation> = HashMap::new();
        if env::var("DDB").is_ok() {
            println!("{}", Backtrace::capture());
        }
        match &self.kind {
            ErrorKind::Mismatch(err) => {
                annotations.insert(
                    err.location,
                    Annotation::new(format!(
                        "Type mismatch, expected {} got {}",
                        err.expected, err.got
                    ))
                    .auto(),
                );
            }
            ErrorKind::Incompatible(err) => {
                annotations.insert(
                    err.location,
                    Annotation::new(format!("{} and {} are incompatible", err.type2, err.type1))
                        .auto(),
                );
            }
            ErrorKind::InvalidOperand(err) => {
                annotations.insert(
                    err.location,
                    Annotation::new({
                        match err.type2 {
                            Some(ty) => format!(
                                "Operand `{}` cannot be used on types {} and {}",
                                err.operand, err.type1, ty
                            ),
                            None => {
                                format!(
                                    "Operand `{}` cannot be used on type {}",
                                    err.operand, err.type1
                                )
                            }
                        }
                    })
                    .auto(),
                );
            }
        }
        let (result, token) = display_annotations(self.file.preparsed, annotations).unwrap();
        let span = token.span();
        writeln!(
            f,
            "{}:{}:{}",
            self.file.preparsed.path.to_str().unwrap(),
            span.line + 1,
            span.line_start + 1 + (span.src_end - span.src_start)
        )?;
        write!(f, "{}", result)
    }
}
impl<'a> TypeError<'a> {
    pub fn new(kind: ErrorKind<'a>, file: &'a SourceFile<'a>) -> Self {
        Self { kind, file }
    }
}
use std::fmt::Write;
pub fn display_diagnostic<'a>(
    file: &'a SourceFile<'a>,
    span: Span,
    message: &str,
) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    let mut annotations: HashMap<Span, parm_ast::error::Annotation> = HashMap::new();
    annotations.insert(span, Annotation::new(message).auto());
    let (annotated, token) = display_annotations(&file.preparsed, annotations).unwrap();
    writeln!(
        result,
        "{}:{}:{}",
        file.preparsed.path.to_str().unwrap(),
        span.line + 1,
        span.line_start + 1 + (span.src_end - span.src_start)
    )?;
    write!(result, "{}", annotated)?;
    Ok(result)
}
