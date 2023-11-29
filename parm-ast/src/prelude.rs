pub use crate::error::{EndOfTokens, ErrorMessage, ExpectedNode, ExpectedToken, ParseError};

pub type ParseResult<'a, T> = Result<T, ParseError<'a>>;

pub use crate::lexer::{token::*, Lexer};
pub use crate::parser::{self, ParseStream, Parser};
pub use crate::traits::Node;
pub use parm_common::{Span, Spanned};
pub use parser::item::Item;
pub use parser::nodes::{
    declaration::{function::Function, variable::Variable},
    delimiter::{Brace, Bracket, Paren},
    expression::{binary::BinaryExpression, number::Number, Expression},
    punctuated::Punctuated,
    statement::{expression_statement::ExpressionStatement, Statement},
    type_nodes::*,
};
