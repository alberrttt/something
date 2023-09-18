use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Punctuated<A: Node, B: Node> {
    pub punctuation: Vec<(A, Option<B>)>,
}
#[test]
fn test_punctuated() {
    let mut parser = Parser::from(TokenStream::from("ident, ident, ident,"));
    let list: (Punctuated<Ident, Comma>, Vec<ParseError>) = <Punctuated<Ident, Comma> as Node<(
        Punctuated<Ident, Comma>,
        Vec<ParseError>,
    )>>::parse(&mut parser);
    dbg!(list);
}
impl<A: Node, B: Node> Node<(Self, Vec<ParseError>)> for Punctuated<A, B> {
    fn parse(parser: &mut Parser) -> (Self, Vec<ParseError>)
    where
        Self: Sized,
    {
        let mut errors = Vec::new();
        let mut punctuation: Vec<(A, Option<B>)> = Vec::new();
        while parser.token_stream.window != 0 && !parser.at_end() {
            match A::parse(parser) {
                Ok(node) => punctuation.push((node, Some(<B as Node>::parse(parser).unwrap()))),
                Err(err) => {
                    B::recover(parser);
                    errors.push(err);
                    let sep = <B as Node>::parse(parser);
                    match sep {
                        Ok(node) => punctuation.push((A::parse(parser).unwrap(), Some(node))),
                        Err(err) => errors.push(err),
                    }
                }
            };
        }
        (Self { punctuation }, errors)
    }

    fn span(&self) -> Span {
        let first = self.punctuation.first().unwrap().0.span();
        let tmp = self.punctuation.last().unwrap();
        let last = match tmp.1 {
            Some(ref node) => node.span(),
            None => tmp.0.span(),
        };
        Span {
            start: first.start,
            end: last.end,
            line: first.line,
            line_start: first.line_start,
        }
    }

    fn append_tokens(&self, to: &mut Vec<Token>) {
        todo!()
    }
}
