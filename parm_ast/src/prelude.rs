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
    declaration::{
        function::{FunctionDeclaration, Param},
        struct_dec::StructDeclaration,
        variable::LetStatement,
    },
    delimiter::{Brace, Bracket, Paren},
    expression::{binary::BinaryExpression, block::Block, call::Call, number::Number, Expression},
    item::{Item, ReturnStatement},
    punctuated::Punctuated,
    statement::{use_stmt::UseStatement, ExpressionWithSemi, Statement},
    type_nodes::*,
};
pub use parser::{ParseStream, Parser};
pub use traits::Node;
