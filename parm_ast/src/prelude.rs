pub use crate::error::{EndOfTokens, ErrorKind, ExpectedNode, ExpectedToken, ParseError};

pub type ParseResult<'a, T> = Result<T, Box<ParseError<'a>>>;

pub use super::source_file::*;
pub use super::tree_display::{Tree, TreeDisplay};
pub use crate::lexer::{token::*, Lexer};
pub use crate::parser::{self, ParseStream, Parser};
pub use crate::traits::Node;
pub use parm_common::{Span, Spanned};
pub use parm_dev_macros::*;
pub use parser::nodes::{
    attribute::Attribute,
    declaration::{
        function::{Function, Param},
        struct_dec::Struct,
        variable::LetStatement,
    },
    delimiter::{Brace, Bracket, Paren},
    expression::{binary::BinaryExpression, call::Call, number::Number, Expression},
    item::Item,
    punctuated::Punctuated,
    statement::{expression_statement::ExpressionStatement, use_stmt::UseStatement, Statement},
    type_nodes::*,
};
pub macro parse($src:expr) {{
    use crate::prelude::*;
    use crate::source_file::PreparsedSourceFile;
    let src = $src;
    dbg!(&src);
    let tokens = Lexer::from(src).lex();
    let mut parser = Parser {
        src,
        tokens,
        current: 0,
    };
    let preparsed = PreparsedSourceFile::new("./test".into(), src);
    (parser, preparsed)
}}
