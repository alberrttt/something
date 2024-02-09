pub mod binary;
pub mod identifier;
use identifier::Identifier;
use parm_ast::{
    lexer::token::StringLiteral,
    parser::nodes::{expression::Boolean, statement::use_stmt::Number},
};
pub mod struct_expression;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a, 'b> {
    Identifier(Identifier<'a, 'b>),
    StringLiteral(&'b StringLiteral<'a>),
    Number(&'b Number<'a>),
    Boolean(&'b Boolean<'a>),
}
