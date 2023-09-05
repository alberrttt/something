use colored::{ColoredString, Colorize};
use something_common::msg::Msg;

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
    #[track_caller]
    pub fn UndefinedIdentifier(ident: Ident, surrounding: TokenStream) -> Self {
        Self {
            surrounding: Some(surrounding),
            kind: TypeErrorKind::UndefinedIdentifier(ident),
            backtrace: Some(Backtrace::capture()),
        }
    }
    #[track_caller]
    pub fn InvalidReturnType(
        expected: Type,
        got: (Type, TokenStream),
        surrounding: TokenStream,
    ) -> Self {
        Self {
            surrounding: Some(surrounding),
            kind: TypeErrorKind::InvalidReturnType { expected, got },
            backtrace: Some(Backtrace::capture()),
        }
    }
    #[track_caller]
    pub fn IncompatibleBinaryOperation(
        left: (Expression, Type),
        right: (Expression, Type),
        operator: Operator,
        surrounding: TokenStream,
    ) -> Self {
        Self {
            surrounding: Some(surrounding),
            kind: TypeErrorKind::IncompatibleBinaryOperation(IncompatibleBinOp {
                left,
                right,
                operator,
            }),

            backtrace: Some(Backtrace::capture()),
        }
    }
    #[track_caller]
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
    #[track_caller]
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
    UndefinedIdentifier(Ident),
    IncompatibleBinaryOperation(IncompatibleBinOp),
    InvalidReturnType {
        expected: Type,
        got: (Type, TokenStream),
    },
}
#[derive(Debug, Clone)]
pub struct IncompatibleBinOp {
    pub left: (Expression, Type),
    pub right: (Expression, Type),
    pub operator: Operator,
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
// create a macro that adds some formatting to the writeln!
macro_rules! err_write {
    (red bold $f:ident, $msg:expr) => {
        write!($f, "{}", $msg.red().bold())
    };
    (red bold $f:ident, $msg:expr, $($args:expr),*) => {
        write!($f, "{}", format!($msg, $($args),*).red().bold())
    };
    (red $f:ident, $msg:expr) => {
        write!($f, "{}", $msg.red())
    };
    (red $f:ident, $msg:expr, $($args:expr),*) => {
        write!($f, "{}", format!($msg, $($args),*).red())
    };
    (yellow $f:ident, $msg:expr) => {
        write!($f, "{}", $msg.yellow())
    };
    (yellow $f:ident, $msg:expr, $($args:expr),*) => {
        write!($f, "{}", format!($msg, $($args),*).yellow())
    };
    ($f:ident, $msg:expr) => {
        write!($f, "{}", $msg)
    };
    ($f:ident, $msg:expr, $($args:expr),*) => {
        write!($f, "{}", format!($msg, $($args),*))
    };
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(backtrace) = self.backtrace.as_ref() {
            // writeln!(f, "{}", backtrace)?;
        }
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
            UndefinedIdentifier(ident) => {
                let msg = Msg::error();
                let msg = msg
                    .header(
                        format!(
                            "undefined identifier: {}",
                            format!("`{}`", ident.name).yellow()
                        )
                        .as_ref(),
                    )
                    .push_body("...")
                    .push_body_w_margin(
                        ColoredString::from(surrounding.to_source_string().as_ref()),
                        ColoredString::from(ident.span.line.to_string().as_ref()),
                    )
                    .push_body(ColoredString::from(
                        format!(
                            "{:offset$}{arrow} {}",
                            "",
                            format!(" undefined identifier `{}`", ident.name)
                                .bright_red()
                                .bold(),
                            offset = ident.span.start - surrounding.first().unwrap().span().start,
                            arrow = "^"
                                .repeat(ident.span.end - ident.span.start)
                                .bright_red()
                                .bold(),
                        )
                        .as_ref(),
                    ));
                write!(f, "{msg}").unwrap();
            }
            Generic(string) => {
                write!(f, "{}", string)?;
            }
            IncompatibleBinaryOperation(IncompatibleBinOp {
                left,
                right,
                operator,
            }) => {
                let right_tkns = right.0.to_tokens();
                let right_start = right_tkns.first().unwrap().span().start;
                let right_end = right_tkns.last().unwrap().span().end;

                let left_tkns = left.0.to_tokens();
                let left_start = left_tkns.first().unwrap().span().start;
                let left_end = left_tkns.last().unwrap().span().end;

                let surrounding = self.surrounding.as_ref().unwrap();
                let line_number = surrounding.0.first().unwrap().span().line;
                let before_offset = left_start - (surrounding.first().unwrap().span().start);
                let msg = Msg::error()
                    .header(
                        format!(
                            "cannot apply operator to operands of type `{}` and `{}`",
                            left.1, right.1
                        )
                        .yellow(),
                    )
                    .push_body("...")
                    .push_body_w_margin(
                        ColoredString::from(surrounding.to_source_string().as_ref()),
                        ColoredString::from(line_number.to_string().as_ref()),
                    )
                    .push_body(ColoredString::from(
                        format!(
                            "{:before_offset$}{}{:offset$}{arrow} {msg}",
                            "",
                            "|".red(),
                            "",
                            before_offset = before_offset,
                            offset = right_end - (left_end + 1),
                            arrow = "^".repeat(right_end - (right_start)).bright_red().bold(),
                            msg = format!(" has type `{}`", right.1).bright_red().bold(),
                        )
                        .as_ref(),
                    ))
                    .push_body(
                        format!(
                            "{:before_offset$}{} has type `{}`",
                            "",
                            "|".red(),
                            left.1,
                            before_offset = before_offset
                        )
                        .red(),
                    );

                write!(f, "{}", msg)?;
            }
            Mismatch(mismatch) => match mismatch {
                TypeMismatch::ExpressionTypeMismatch((expression, infered_type), expected_type) => {
                    let expr_tkns = expression.to_tokens();
                    let expr_start = expr_tkns.first().unwrap().span().start;
                    let expr_end = expr_tkns.last().unwrap().span().end;
                    let surrounding = self.surrounding.as_ref().unwrap();
                    let msg = Msg::error()
                        .header(
                            format!(
                                "{} expected `{expected_type}` but got `{infered_type}`",
                                "type mismatch".bright_red().bold()
                            )
                            .yellow(),
                        )
                        .push_body("...")
                        .push_body_w_margin(
                            surrounding.to_source_string().white().clear(),
                            expr_tkns
                                .first()
                                .unwrap()
                                .span()
                                .line_start
                                .to_string()
                                .white()
                                .clear(),
                        )
                        .push_body(
                            format!(
                                "\t{:offset$}{arrow} {}",
                                "",
                                format!(" has type {}", infered_type).bright_red().bold(),
                                offset =
                                    expr_start - (surrounding.first().unwrap().span().start + 8),
                                arrow = "^".repeat(expr_end - expr_start).bright_red().bold()
                            )
                            .white(),
                        );

                    write!(f, "{}", msg)?;
                }
            },
            InvalidReturnType { expected, got } => {
                let arrow_start = got.1.first().unwrap().span().start;
                let arrow_end = got.1.last().unwrap().span().end;

                let surrounding_start = surrounding.first().unwrap().span().start;
                let offset = arrow_start - surrounding_start;

                let msg = Msg::error()
                    .header(
                        format!("expected return type `{}` but got `{}`", expected, got.0).yellow(),
                    )
                    .push_body("...")
                    .push_body_w_margin(
                        ColoredString::from(surrounding.to_source_string().as_ref()),
                        ColoredString::from(
                            surrounding
                                .first()
                                .unwrap()
                                .span()
                                .line
                                .to_string()
                                .as_ref(),
                        ),
                    )
                    .push_body(
                        format!(
                            "{:offset$}{arrow} has type `{}`",
                            "",
                            got.0,
                            offset = offset,
                            arrow = "^".repeat(arrow_end - arrow_start).bright_red(),
                        )
                        .red(),
                    );

                write!(f, "{msg}")?;
            }
        }
        writeln!(f)
    }
}
impl std::error::Error for TypeError {}
