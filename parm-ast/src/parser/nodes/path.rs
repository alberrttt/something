use parm_common::Spanned;
use parm_dev_macros::Spanned;

use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub enum SimpleSegment<'a> {
    Identifier(Identifier<'a>),
    USelf(USelf<'a>),
    All(Asterisk<'a>),
}
impl<'a> Node<'a> for SimpleSegment<'a> {
    fn parse(parse_stream: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let peeked = parse_stream.peek()?;
        match peeked {
            Token::Identifier(_) => Ok(Self::Identifier(Identifier::parse(parse_stream)?)),
            Token::USelf(_) => Ok(Self::USelf(USelf::parse(parse_stream)?)),
            Token::Asterisk(_) => Ok(Self::All(Asterisk::parse(parse_stream)?)),
            _ => ParseError::err(
                crate::error::ErrorKind::ExpectedNode(crate::error::ExpectedNode {
                    got: parse_stream.peek()?.lexeme(),
                    expected: "a path segment",
                    location: parse_stream.current,
                }),
                parse_stream.tokens,
                parse_stream.src_file,
            ),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct SimplePath<'a> {
    pub prefix: Option<ColonColon<'a>>,
    pub segments: Punctuated<SimpleSegment<'a>, ColonColon<'a>>,
}
impl<'a> Node<'a> for SimplePath<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let prefix = parser.step(ColonColon::parse).ok();
        let segments = parser.parse()?;
        Ok(Self { prefix, segments })
    }
}
