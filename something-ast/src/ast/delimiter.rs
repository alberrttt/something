use super::prelude::*;
use crate::tokenizer::prelude::*;
macro_rules! gen_impl {
    ($ident: ident, $variant:ident) => {
        impl<T> Parse for $ident<T>
        where
            T: Parse,
        {
            #[track_caller]
            fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self>
            where
                Self: Sized,
            {
                if let Ok(Token::$variant(_)) = parser.peek() {
                    let Token::$variant(tmp) = parser.advance()?.clone() else {
                                                        unsafe {
                                                            std::hint::unreachable_unchecked();
                                                        }
                                                    };
                    let inner = Parse::parse(parser).unwrap();
                    return Ok(Self {
                        opening: tmp,
                        inner,
                        closing: Parse::parse(parser)?,
                    });
                } else {
                    Recoverable
                }
            }
        }
        impl<T> AppendTokens for $ident<T>
        where
            T: AppendTokens + Clone,
        {
            fn append_tokens(&self, tokens: &mut TokenStream) {
                self.opening.clone().append_tokens(tokens);
                self.inner.clone().append_tokens(tokens);
                self.closing.clone().append_tokens(tokens);
            }
        }
        impl<T> ParsingDisplay for $ident<T>
        where
            T: ParsingDisplay,
        {
            fn display(&self) -> String
            where
                Self: Sized,
            {
                format!(
                    "{}{}{}",
                    self.opening.display(),
                    self.inner.display(),
                    self.closing.display()
                )
            }

            fn placeholder() -> String
            where
                Self: Sized,
            {
                todo!()
            }
        }
        impl<T> Deref for $ident<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl<T> std::ops::DerefMut for $ident<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
}
use std::ops::Deref;
#[derive(Debug, Clone)]
pub struct Paren<T> {
    pub opening: LeftParen,
    pub inner: T,
    pub closing: RightParen,
}
#[derive(Debug, Clone)]
pub struct Brace<T> {
    pub opening: LeftBrace,
    pub inner: T,
    pub closing: RightBrace,
}
#[derive(Debug, Clone)]
pub struct Bracket<T> {
    pub opening: LeftBracket,
    pub inner: T,
    pub closing: RightBracket,
}

gen_impl!(Paren, LeftParen);
gen_impl!(Brace, LeftBrace);
gen_impl!(Bracket, LeftBracket);
