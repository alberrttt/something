#[macro_export]
macro_rules! tkn_recover {
    (eot $expr:expr) => {
        match $expr {
            Ok(x) => Ok(x),
            Err(_) | Recoverable => Recoverable,
        }
    };
}
#[macro_export]
/// imitates the `matches!` macro
macro_rules! peek_matches {
    ($self:ident, $($pat:pat_param)|+) => {
        match $self.peek() {
            Ok(token) => match token {
                $($pat)|+ => true,
                _ => false,
            },
            _ => false,
        }
    };
}
#[macro_export]
macro_rules! node {
    ($typ:ty, $string:literal) => {{
        let tokens = TokenStream::from($string);
        let mut parser = Parser::from(tokens);
        let node = <$typ as Node>::parse(&mut parser).unwrap();
        node
    }};
    ($typ:ty, $parser:expr) => {{
        let node = <$typ as Node>::parse($parser).unwrap();
        node
    }};
    (tokens $typ:ty, $tokenstream:expr) => {{
        let mut parser = Parser::from($tokenstream);
        let node = <$typ as Node>::parse(&mut parser).unwrap();
        node
    }};
}
