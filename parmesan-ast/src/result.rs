#[derive(Debug, Clone, PartialEq)]
pub enum PResult<'a, T> {
    Ok(T),
    Err(crate::error::ParseError<'a>),
}

use std::{cell::UnsafeCell, fmt::Debug, ops::*};

use crate::{error::ParseError, parser::Parser};
impl<'a, T> PResult<'a, T>
where
    T: Debug + Clone,
{
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            PResult::Ok(t) => t,
            PResult::Err(err) => panic!("called `PResult::unwrap()` on an `Err` value: {:?}", err),
        }
    }
}
impl<'a, T> FromResidual<crate::error::ParseError<'a>> for PResult<'a, T>
where
    T: Debug + Clone,
{
    fn from_residual(residual: crate::error::ParseError<'a>) -> Self {
        PResult::Err(residual)
    }
}

impl<'a, T> Try for PResult<'a, T>
where
    T: Debug + Clone,
{
    type Output = T;

    type Residual = ParseError<'a>;

    fn from_output(output: Self::Output) -> Self {
        PResult::Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            PResult::Ok(output) => ControlFlow::Continue(output),
            PResult::Err(residual) => ControlFlow::Break(residual),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::{token::Token, Lexer},
        parser::Parser,
    };

    use super::PResult;

    #[derive(Debug, Clone, PartialEq)]
    struct Foo;
    impl Foo {
        fn parse<'a>(parser: &'a mut Parser) -> PResult<'a, Self> {
            use PResult::*;
            parser.step(|parser| {
                let adv = parser.advance_1()?;
                match adv {
                    Token::Ident(ident) => {
                        if ident.lexeme == "foo" {
                            return Ok(Foo);
                        }
                    }
                    _ => {}
                }
                return Err(crate::error::ParseError::EndOfTokens(
                    crate::error::EndOfTokens {},
                ));
            })
        }
    }
    #[test]
    fn main() {
        let src = "foo+2;";
        let mut lexer = Lexer::from(src);
        let mut tokens = lexer.lex();
        let mut parser = Parser {
            src,
            tokens: &tokens,
            current: 0,
        };
        Foo::parse(&mut parser);
        assert_eq!(parser.current, 1);
    }
}
