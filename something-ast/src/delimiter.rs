use something_frontend_tokenizer::{
    delimiter::Delimiter,
    tokens::{Parse, Token},
    Tokens,
};
#[derive(Debug, Clone)]
pub struct Parenthesis<T>(pub Delimiter<'(', ')'>, pub Option<T>);
impl<T> Parse for Parenthesis<T>
where
    T: Parse,
{
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Token::Paren(paren) = input.advance() else {
            panic!()
        };

        Ok(Self(paren.clone(), {
            if !paren.tokens.is_empty() {
                let mut inner = Tokens::from(paren.tokens.clone());
                Some(Parse::parse(&mut inner)?)
            } else {
                None
            }
        }))
    }
}

#[derive(Debug, Clone)]
pub struct Brackets<T>(pub Delimiter<'[', ']'>, pub T);
impl<T> Parse for Brackets<T>
where
    T: Parse,
{
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Token::Bracket(bracket) = input.advance() else {
            panic!()
        };
        let mut inner = Tokens::from(bracket.tokens.clone());
        let value = Parse::parse(&mut inner).unwrap();
        Ok(Self(bracket.clone(), value))
    }
}

#[derive(Debug, Clone)]
pub struct Braces<T>(pub Delimiter<'{', '}'>, pub T);
impl<T> Parse for Braces<T>
where
    T: Parse,
{
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Token::Brace(braces) = input.advance() else {
            panic!()
        };
        let mut inner = Tokens::from(braces.tokens.clone());
        let value = Parse::parse(&mut inner).unwrap();
        Ok(Self(braces.clone(), value))
    }
}
