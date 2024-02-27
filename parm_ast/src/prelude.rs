use crate::*;
pub use error::{EndOfTokens, ErrorKind, ExpectedNode, ExpectedToken, ParseError};

pub type ParseResult<'a, T> = Result<T, Box<ParseError<'a>>>;

pub use super::source_file::*;
pub use super::tree_display::{Tree, TreeDisplay};
pub use lexer::{token::*, Lexer};
pub use parm_common::{Span, Spanned};
pub use parm_dev_macros::*;
pub use parser::nodes::{
    attribute::Attribute,
    delimiter::{Brace, Bracket, Paren},
    expression::{binary::BinaryExpression, block::Block, call::Call, number::Number, Expression},
    item::{
        function::FunctionDeclaration, struct_dec::StructDeclaration, use_stmt::UseStatement, Item,
    },
    punctuated::Punctuated,
    statement::{ret::ReturnStatement, variable::LetStatement, ExpressionWithSemi, Statement},
    type_nodes::*,
};
pub use parser::{ParseStream, Parser};
pub use traits::Node;
