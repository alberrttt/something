use crate::prelude::*;
use parm_common::Spanned;
use std::ops::Deref;
macro_rules! Delimiter {
    ($name:ident,$open:ident,$close:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name<'a, T: Node<'a> + Spanned> {
            pub open: $open<'a>,
            pub inner: T,
            pub close: $close<'a>,
        }
        impl<'a, T: Spanned + Node<'a> + TreeDisplay> TreeDisplay for $name<'a, T> {
            fn tree(&self) -> Tree {
                Tree::new(stringify!($name))
                    .child(self.inner.tree())
                    .label("inner")
            }
        }
        impl<'a, T: Node<'a>> Deref for $name<'a, T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl<'a, T: Spanned + Node<'a>> Spanned for $name<'a, T> {
            fn span(&self) -> parm_common::Span {
                (self.open.span(), self.close.span()).into()
            }
        }
        impl<'a, T: Spanned + Node<'a>> $name<'a, T> {
            pub fn parse_manual(
                parser: &mut ParseStream<'a>,
                parsing: fn(&mut ParseStream<'a>) -> ParseResult<'a, T>,
            ) -> ParseResult<'a, Self> {
                let open = parser.step($open::parse)?;
                let mut depth = 1;
                let start = parser.current;
                use std::borrow::BorrowMut;
                while depth > 0 {
                    match match parser.peek() {
                        Ok(token) => token,
                        Err(mut err) => {
                            parser.panic = true;
                            match err.kind.borrow_mut() {
                                ErrorKind::EndOfTokens(ref mut eot) => {
                                    eot.expected = Some(stringify!($close));
                                    return Err(err);
                                }
                                _ => {
                                    return Err(err);
                                }
                            }
                        }
                    } {
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
                let inner = parsing(&mut inner_parse_stream)?;
                let close = if inner_parse_stream.at_end() {
                    match parser.step($close::parse) {
                        Ok(close) => close,
                        Err(err) => {
                            parser.panic = inner_parse_stream.panic;
                            return Err(err);
                        }
                    }
                } else {
                    match inner_parse_stream.step($close::parse) {
                        Ok(close) => close,
                        Err(err) => {
                            parser.panic = inner_parse_stream.panic;
                            return Err(err);
                        }
                    }
                };
                Ok(Self { open, inner, close })
            }
        }
        impl<'a, T: Spanned + Node<'a>> Node<'a> for $name<'a, T> {
            fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
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
// #[test]
// fn test_delimiter() {
//     let mut parser = Parser::new("[{abc}]");
//     let mut paren = <Bracket<Brace<Identifier>> as Node>::parse(&mut parser.stream()).unwrap();
//     dbg!(&paren);
// }
