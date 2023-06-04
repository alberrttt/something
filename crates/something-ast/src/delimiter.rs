use super::prelude::*;
use something_frontend_tokenizer::error::ParseError;
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
                    input: &mut something_frontend_tokenizer::Tokens,
                ) -> Result<Self, ParseError> {
                    let Some(Token::$delimiter(delimiter)) = input.advance() else {
                        return Err(
                            ParseError::ExpectedToken(
                                Token::$delimiter(Default::default()),
                                input.advance().cloned().unwrap(),
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
    T: std::fmt::Debug + Clone + something_frontend_tokenizer::ParsingDisplay,
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
    T: std::fmt::Debug + Clone + something_frontend_tokenizer::ParsingDisplay,
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
    T: std::fmt::Debug + Clone + something_frontend_tokenizer::ParsingDisplay,
{
    fn display(&self) -> String {
        format!("({})", self.1.display())
    }
    fn placeholder() -> String {
        format!("{}{}", stringify!($delimiter), T::placeholder())
    }
}
