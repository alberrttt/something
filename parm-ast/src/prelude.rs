pub use crate::error::{EndOfTokens, ErrorKind, ExpectedNode, ExpectedToken, ParseError};

pub type ParseResult<'a, T> = Result<T, ParseError<'a>>;

pub use crate::lexer::{token::*, Lexer};
pub use crate::parser::{self, ParseStream, Parser};
pub use crate::traits::Node;
pub use parm_common::{Span, Spanned};
pub use parm_dev_macros::*;
pub use parser::nodes::{
    attribute::Attribute,
    declaration::{function::Function, variable::Variable},
    delimiter::{Brace, Bracket, Paren},
    expression::{binary::BinaryExpression, number::Number, Expression},
    item::Item,
    punctuated::Punctuated,
    statement::{expression_statement::ExpressionStatement, use_stmt::UseStatement, Statement},
    type_nodes::*,
};
