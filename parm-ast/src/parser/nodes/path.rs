use parm_common::Spanned;
use parm_dev_macros::Spanned;

use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Spanned)]
pub enum SimpleSegment<'a> {
    Identifier(Identifier<'a>),
    USelf(USelf<'a>),
    All(Asterisk<'a>),
}
impl<'a> Node<'a> for SimpleSegment<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let peeked = parser.peek()?;
        match peeked {
            Token::Identifier(_) => Ok(Self::Identifier(Identifier::parse(parser)?)),
            Token::USelf(_) => Ok(Self::USelf(USelf::parse(parser)?)),
            Token::Asterisk(_) => Ok(Self::All(Asterisk::parse(parser)?)),
            _ => Err(ParseError::new(
                crate::error::ErrorKind::ExpectedNode(crate::error::ExpectedNode {
                    got: parser.peek()?.lexeme(),
                    expected: "a path segment",
                    location: parser.current,
                }),
                parser.tokens,
            )),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct SimplePath<'a> {
    pub prefix: Option<ColonColon<'a>>,
    pub segments: Punctuated<SimpleSegment<'a>, ColonColon<'a>>,
}
impl<'a> Node<'a> for SimplePath<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let prefix = parser.step(ColonColon::parse).ok();
        let segments = parser.parse()?;
        Ok(Self { prefix, segments })
    }
}
