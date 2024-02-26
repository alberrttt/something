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
                    location: peeked.span(),
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
impl<'a> SimplePath<'a> {
    pub fn first_segment(&self) -> &SimpleSegment<'a> {
        let eles = self.segments.elements();
        assert_eq!(eles.len(), 1);
        eles.first().unwrap()
    }
    pub fn from_ident(ident: Identifier<'a>) -> Self {
        Self {
            prefix: None,
            segments: Punctuated::from_single(SimpleSegment::Identifier(ident)),
        }
    }
    pub fn from_pair(segment: SimpleSegment<'a>, sep: ColonColon<'a>) -> Self {
        Self {
            prefix: None,
            segments: Punctuated {
                inner: vec![(segment, sep)],
                last: None,
            },
        }
    }
    pub fn parse_more(self, stream: &mut ParseStream<'a>) -> ParseResult<'a, Self> {
        let punct = self.segments;

        let segments = punct.parse_terminated(stream)?;
        Ok(Self {
            prefix: self.prefix,
            segments,
        })
    }
    pub fn start_with_ident(ident: Identifier<'a>, parse_stream: &mut ParseStream) -> Self {
        Self {
            prefix: None,
            segments: Punctuated::from_single(SimpleSegment::Identifier(ident)),
        }
    }
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
