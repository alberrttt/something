use parmesan_dev_macros::Spanned;

use crate::prelude::*;
use parmesan_common::Spanned;

macro_rules! Delimiter {
    ($name:ident,$open:ident,$close:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name<'a, T: Node<'a> + Spanned> {
            pub open: $open<'a>,
            pub inner: T,
            pub close: $close<'a>,
        }

        impl<'a, T: Spanned + Node<'a>> Spanned for $name<'a, T> {
            fn span(&self) -> parmesan_common::Span {
                (self.open.span(), self.close.span()).into()
            }
        }
        impl<'a, T: Spanned + Node<'a>> $name<'a, T> {
            pub fn parse_manual(
                parser: &mut ParseStream<'a>,
                parsing: fn(&mut ParseStream<'a>) -> ParseResult<'a, T>,
            ) -> Result<Self, crate::error::ParseError<'a>> {
                let open = parser.step($open::parse)?;
                let mut depth = 1;
                let start = parser.current;
                while depth > 0 {
                    match parser.peek()? {
                        Token::$open(_) => {
                            parser.advance()?;
                            depth += 1;
                        }
                        Token::$close(_) => {
                            depth -= 1;
                        }
                        _ => {
                            parser.advance()?;
                        }
                    }
                }
                let mut inner_parse_stream = parser.from_range(start..parser.current);
                dbg!(&inner_parse_stream);
                let inner = inner_parse_stream.step(parsing)?;
                let close = if inner_parse_stream.at_end() {
                    parser.step($close::parse)?
                } else {
                    inner_parse_stream.step($close::parse)?
                };
                Ok(Self { open, inner, close })
            }
        }
        impl<'a, T: Spanned + Node<'a>> Node<'a> for $name<'a, T> {
            fn parse(
                parser: &mut crate::parser::ParseStream<'a>,
            ) -> Result<Self, crate::error::ParseError<'a>>
            where
                Self: Sized,
            {
                $name::parse_manual(parser, T::parse)
            }
        }
    };
}
Delimiter!(Paren, LParen, RParen);
Delimiter!(Bracket, LBracket, RBracket);
Delimiter!(Brace, LBrace, RBrace);
Delimiter!(Angle, Less, Greater);
#[test]
fn test_delimiter() {
    let mut parser = Parser::new("[{abc}]");
    let mut paren = <Bracket<Brace<Ident>> as Node>::parse(&mut parser.stream()).unwrap();
    dbg!(&paren);
}
