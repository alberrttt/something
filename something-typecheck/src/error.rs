use colored::Colorize;

use crate::ast::prelude::*;
use crate::scopes::CheckType;
use crate::symbol::Type;
use crate::tokenizer::prelude::*;

pub struct TypeError {
    pub surrounding: Option<TokenStream>,
    pub kind: TypeErrorKind,
    pub backtrace: Option<Backtrace>,
}
//TODO
#[derive(Debug, Clone)]
pub struct SurroundingTokensPayload {
    pub tokens: TokenStream,
    pub range: Range<u16>,
}
impl SurroundingTokensPayload {
    pub fn new(tokens: TokenStream, range: Range<u16>) -> Self {
        Self { tokens, range }
    }
}
impl Debug for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeError")
            .field("surrounding", &self.surrounding)
            .field("kind", &self.kind)
            .finish()
    }
}
impl Clone for TypeError {
    fn clone(&self) -> Self {
        Self {
            surrounding: self.surrounding.clone(),
            kind: self.kind.clone(),
            backtrace: match &self.backtrace {
                Some(backtrace) => {
                    panic!();
                }
                None => None,
            },
        }
    }
}
#[allow(non_snake_case)]
impl TypeError {
    pub fn IncompatibleBinaryOperation(
        left: (Expression, Type),
        right: (Expression, Type),
    ) -> Self {
        Self::create(
            &left.0,
            TypeErrorKind::IncompatibleBinaryOperation {
                left: left.clone(),
                right,
            },
        )
    }
    pub fn MismatchExpressionType(
        expression: Expression,
        infered: Option<Type>,
        expected: Type,
    ) -> Self {
        Self::create(
            &expression,
            TypeErrorKind::Mismatch(TypeMismatch::ExpressionTypeMismatch(
                (
                    expression.clone(),
                    match infered {
                        Some(some) => some,
                        None => expression.resolve_type(None, None).unwrap(),
                    },
                ),
                expected,
            )),
        )
    }
    pub fn Generic(msg: impl Into<String>) -> Self {
        Self::create(
            &Token::Eof(Default::default()),
            TypeErrorKind::Generic(msg.into()),
        )
    }
}
impl TypeError {
    fn create(surrounding_tokens: &dyn AppendTokens, kind: TypeErrorKind) -> Self {
        let mut tokenstream = TokenStream::default();
        surrounding_tokens.append_tokens(&mut tokenstream);
        Self {
            surrounding: Some(tokenstream),
            kind,

            backtrace: Some(Backtrace::capture()),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeErrorKind {
    Generic(String),
    Mismatch(TypeMismatch),
    IncompatibleBinaryOperation {
        left: (Expression, Type),
        right: (Expression, Type),
    },
}
#[derive(Debug, Clone)]
pub enum TypeMismatch {
    ExpressionTypeMismatch(
        (Expression, Type), // <- expression, and infered type
        Type,               // <- expected type
    ),
}
use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt::Debug;
use std::ops::Range;
use std::rc::Rc;
use std::{any, default};
#[derive(Debug, Clone)]
pub struct ExpectedAst {
    pub ast: any::TypeId,
}
#[derive(Debug, Clone)]
pub struct ExpectedToken {
    pub expected: Token,
    pub at: usize, // <- an index to `surrounding`
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "error ".red().bold())?;
        let surrounding = self.surrounding.as_ref().unwrap();
        match &self.backtrace {
            Some(b) => {
                if std::env::var("ERR_BACKTRACE").unwrap_or_default() == "1" {
                    match &self.backtrace {
                        Some(backtrace) => {
                            writeln!(f, "\n{}", backtrace)?;
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
        use TypeErrorKind::*;
        match &self.kind {
            Generic(string) => {
                write!(f, "{}", string)
            }
            IncompatibleBinaryOperation { left, right } => {
                // TODO
                writeln!(
                    f,
                    "cannot apply operator to operands of type `{}` and `{}`",
                    left.1, right.1,
                )
            }
            Mismatch(mismatch) => match mismatch {
                TypeMismatch::ExpressionTypeMismatch((expression, infered_type), expected_type) => {
                    let expr_tkns = expression.to_tokens();
                    let expr_start = expr_tkns.first().unwrap().span().start;
                    let expr_end = expr_tkns.last().unwrap().span().end;
                    let surrounding = self.surrounding.as_ref().unwrap();
                    writeln!(
                        f,
                        "{}: {}",
                        "type mismatch".bright_red().bold(),
                        format!("expected `{expected_type}` but got `{infered_type}`").yellow(),
                    )?;
                    writeln!(
                        f,
                        "{}\t...\n{x}\t{}",
                        "   |".red(),
                        surrounding.to_source_string(),
                        x = format!(" {} |", expr_tkns.first().unwrap().span().line).red()
                    )?;
                    writeln!(
                        f,
                        "\t{:offset$}{arrow} {}",
                        "",
                        format!(" has type {}", infered_type).bright_red().bold(),
                        offset = expr_start - surrounding.first().unwrap().span().start,
                        arrow = "^".repeat(expr_end - expr_start).bright_red().bold(),
                    )
                }
            },
        }
    }
}
impl std::error::Error for TypeError {}
