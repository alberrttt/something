use super::TokenStream;
use crate::prelude::*;
use crate::Parser;

use crate::ast;
use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;
pub trait Node {
    fn parse(parser: &mut Parser) -> ParseResult<Self>
    where
        Self: Sized;
    fn span(&self) -> Span {
        todo!()
    }
    fn into_tokens(&self) -> Vec<Token> {
        todo!()
    }
}
