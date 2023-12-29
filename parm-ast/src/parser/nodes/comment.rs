use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Comment<'a> {
    pub slash_slash: SlashSlash<'a>,
    pub text: &'a [Token<'a>],
}
impl<'a> Node<'a> for Comment<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let slash_slash = parser.step(|parser| SlashSlash::parse(parser).clone())?;
        let starting_line = slash_slash.span().line;
        let starting = parser.current;
        while let Ok(token) = parser.peek() {
            if token.span().line == starting_line {
                parser.step(|parser| parser.advance())?;
            } else {
                break;
            }
        }
        Ok(Self {
            slash_slash,
            text: &parser.tokens[starting..parser.current],
        })
    }
}
