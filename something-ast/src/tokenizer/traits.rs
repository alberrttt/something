use super::TokenStream;
use crate::ast::delimiters::Paren;
use crate::prelude::*;
use crate::Parser;

use crate::ast;
use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;
pub trait Node<Output = ParseResult<Self>> {
    fn parse(parser: &mut Parser) -> Output
    where
        Self: Sized;
    fn recover(parser: &mut Parser)
    where
        Self: Sized,
    {
        todo!()
    }
    fn span(&self) -> Span;
    fn append_tokens(&self, to: &mut Vec<Token>);
}
// basicall, if an error has occured, we will consume the rest of the tokens to clean it up
#[test]
fn test_paren_list() {
    let idents = TokenStream::from("(ident ident ident)");
    let list: Paren<Vec<Ident>> = Parser::from(idents).parse().unwrap();
    dbg!(&list);
}
impl<T: Node> Node for Vec<T> {
    fn parse(parser: &mut Parser) -> ParseResult<Self>
    where
        Self: Sized,
    {
        let mut nodes = Vec::new();
        while parser.token_stream.window != 0 {
            let node = T::parse(parser)?;
            nodes.push(node);
        }
        Ok(nodes)
    }

    fn recover(parser: &mut Parser)
    where
        Self: Sized,
    {
        todo!()
    }

    fn span(&self) -> Span {
        let first = self.first().unwrap();
        let last = self.last().unwrap_or(first);

        Span {
            start: first.span().start,
            end: last.span().end,
            line: first.span().line,
            line_start: first.span().line_start,
        }
    }

    fn append_tokens(&self, to: &mut Vec<Token>) {
        for node in self {
            node.append_tokens(to);
        }
    }
}
