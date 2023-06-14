use super::prelude::*;
use crate::tokenizer::prelude::*;
macro_rules! delimiter_impl {
    [$($delimiter:ident),*] => {
        $(
            #[derive(Debug, Clone)]
            pub struct $delimiter<T>(pub Span, pub T);
            impl<T> Default for $delimiter<T> where T: Default {
                fn default() -> Self {
                    Self(Span::default(), T::default())
                }
            }
            impl<T> AppendTokens for $delimiter<T>
            where
                T: AppendTokens,
            {
                fn append_tokens(&self, tokens: &mut Tokens)
                where
                    Self: Sized,
                {
                    let mut tmp = Tokens::new();
                    self.1.append_tokens(&mut tmp);
                    // its so late and im lazy to implement a more idiomatic way to get the span
                    let tmp = Token::$delimiter(Delimiter {
                        tokens: tmp.0.clone(),
                        span: tmp.0.first().unwrap().span(),
                    });
                    tokens.push(tmp);
                }
            }
            impl<T> Deref for $delimiter<T> {
                type Target = T;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T> Parse for $delimiter<T>
            where
                T: Parse,
                T: std::fmt::Debug + Clone,
            {
                fn parse(
                    input: &mut Tokens,
                ) -> ParseResult<Self> {
                    let tmp = input.advance()?;
                    let Token::$delimiter(delimiter) = tmp else {
                        return Err(
                            ParseError::ExpectedToken(
                                Token::$delimiter(Default::default()),
                                tmp.clone(),
                            )
                        );
                    };
                    let tokens = &mut delimiter.tokens.clone().into();
                    let tmp = Parse::parse(tokens)?;
                    if !tokens.at_end() {
                        return Err(ParseError::Generic(format!("Did not expect these tokens: {:?}", &tokens.0[tokens.1..]).into()));
                    }
                    Ok(Self(delimiter.span, tmp))
                }
            }
        )*
    };
    () => {

    };
}
use something_dev_tools::item_name;
use std::ops::Deref;
delimiter_impl![Braces, Brackets, Parentheses];

impl<T> ParsingDisplay for Brackets<T>
where
    T: std::fmt::Debug + Clone + ParsingDisplay,
{
    fn display(&self) -> String {
        format!("[{}]", self.1.display())
    }
    fn placeholder() -> String {
        format!("{}{}", stringify!($delimiter), T::placeholder())
    }
}

impl<T> ParsingDisplay for Braces<T>
where
    T: std::fmt::Debug + Clone + ParsingDisplay,
{
    fn display(&self) -> String {
        format!("{{{}}}", self.1.display())
    }
    fn placeholder() -> String {
        format!("{}{}", stringify!($delimiter), T::placeholder())
    }
}
impl<T> ParsingDisplay for Parentheses<T>
where
    T: std::fmt::Debug + Clone + ParsingDisplay,
{
    fn display(&self) -> String {
        format!("({})", self.1.display())
    }
    fn placeholder() -> String {
        format!("{}{}", stringify!($delimiter), T::placeholder())
    }
}
