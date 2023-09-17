use crate::*;

macro_rules! create_delimiter {
    ($name:ident, $left:ident, $right:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name<T: Node> {
            pub open: $left,
            pub inner: T,
            pub close: $right,
        }

        impl<T: Node> Node for $name<T> {
            fn parse(parser: &mut Parser) -> ParseResult<Self>
            where
                Self: Sized,
            {
                parser.in_delimiter = true;
                let Ok(first) = <$left as Node>::parse(parser) else {
                    panic!("parse error")
                };
                let window = parser.token_stream.window;
                let count = || {
                    let mut count = 0;
                    let mut depth = 0;
                    while let Ok(token) = parser.peek_n(count) {
                        match token {
                            Token::$left(_) => {
                                depth += 1;
                            }
                            Token::$right(_) => {
                                if depth == 0 {
                                    return count;
                                }
                                depth -= 1;
                            }
                            _ => {}
                        }
                        count += 1;
                    }
                    count
                };
                let count = count();
                if count == 0 {
                    <$right as Node>::parse(parser)?;
                    panic!("empty");
                }
                parser.token_stream.window = count;
                dbg!(parser.token_stream.peek());

                let inner = match parser.step(|f| <T as Node>::parse(f)) {
                    Ok(v) => v,
                    Err(e) => {
                        <T as Node>::recover(parser);
                        return Err(e);
                    }
                };
                parser.token_stream.window = window;
                let close = match parser.step(|f| <$right as Node>::parse(f)) {
                    Ok(v) => v,
                    Err(e) => {
                        <$right as Node>::recover(parser);
                        return Err(e);
                    }
                };
                parser.in_delimiter = false;
                Ok(Self {
                    open: first,
                    inner,
                    close,
                })
            }
            fn span(&self) -> Span {
                Span {
                    start: self.open.span().start,
                    end: self.open.span().end,
                    line: self.open.span().line,
                    line_start: self.close.span().line_start,
                }
            }
            fn append_tokens(&self, to: &mut Vec<Token>) {
                self.open.append_tokens(to);
                self.inner.append_tokens(to);
                self.close.append_tokens(to);
            }
            fn recover(parser: &mut Parser)
            where
                Self: Sized,
            {
            }
        }
    };
}
create_delimiter!(Paren, LeftParen, RightParen);
create_delimiter!(Brace, LeftBrace, RightBrace);
create_delimiter!(Bracket, LeftBracket, RightBracket);
