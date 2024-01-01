use std::borrow::BorrowMut;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct StringLit<'a> {
    pub left_quote: DoubleQuote<'a>,
    pub value: &'a str,
    pub right_quote: DoubleQuote<'a>,
}
impl<'a> TreeDisplay for StringLit<'a> {
    fn tree(&self) -> Tree {
        Tree::new("StringLit").lexeme(format!("{}", self.value))
    }
}
impl<'a> Node<'a> for StringLit<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let left_quote = DoubleQuote::parse(parser)?;
        let start = parser.current;
        loop {
            match parser.peek() {
                Ok(Token::DoubleQuote(_)) => break,
                Ok(_) => {}
                Err(mut err) => match err.kind.borrow_mut() {
                    ErrorKind::EndOfTokens(eot) => {
                        eot.expected = Some("\"");
                        return Err(err);
                    }
                    _ => {
                        return Err(err);
                    }
                },
            }

            parser.advance()?;
        }
        let value = &parser.src_text()[start..parser.current];
        let right_quote = DoubleQuote::parse(parser)?;
        Ok(Self {
            left_quote,
            value,
            right_quote,
        })
    }
}
