use something_frontend_tokenizer::{
    delimiter::Delimiter,
    tokens::{Parse, Span, Token},
    Tokens,
};
macro_rules! delimiter_impl {
    [$($delimiter:ident),*] => {
        $(
            #[derive(Debug, Clone)]
            pub struct $delimiter<T>(pub Span, pub T);
            impl<T> Parse for $delimiter<T>
            where
                T: Parse,
                T: std::fmt::Debug,
            {
                fn parse(
                    input: &mut something_frontend_tokenizer::Tokens,
                ) -> Result<Self, Box<dyn std::error::Error>> {
                    let Some(Token::$delimiter(delimiter)) = input.advance() else {
                        return Err(format!("expected {} but got: {:?}", stringify!($delimiter), input.peek()).into());
                    };
                    let tokens = &mut delimiter.tokens.clone().into();
                    let tmp = Parse::parse(tokens)?;
                    if !tokens.at_end() {
                        return Err(format!("Did not expect these tokens: {:?}", &tokens.0[tokens.1..]).into());
                    }
                    Ok(Self(delimiter.span, tmp))
                }
            }
        )*
    };
    () => {

    };
}
delimiter_impl![Braces, Brackets, Parentheses];
