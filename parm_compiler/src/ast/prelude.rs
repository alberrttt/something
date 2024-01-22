use crate::ast::*;
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
        function::{Function, Param},
        struct_dec::Struct,
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
pub macro parse($src:expr) {{
    use crate::ast::prelude::*;
    use crate::ast::source_file::PreparsedSourceFile;
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
