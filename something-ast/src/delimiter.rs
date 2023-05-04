use something_frontend_tokenizer::{
    delimiter::Delimiter,
    tokens::{Parse, Span, Token},
    Tokens,
};
#[derive(Debug, Clone)]
pub struct Parenthesis<T>(pub Span, pub T);
impl<T> Parse for Parenthesis<T>
where
    T: Parse,
{
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Some( Token::Paren(paren)) = input.advance() else {
            return Err(format!("Expected Parenthesis, got {:?}", input.advance().clone()).into())
        };
        Ok(Self(
            paren.span,
            Parse::parse(&mut (paren.tokens.clone()).into())?,
        ))
    }
}
#[derive(Debug, Clone)]
pub struct Brackets<T>(pub Span, pub T);
impl<T> Parse for Brackets<T>
where
    T: Parse,
{
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Some(Token::Bracket(brackets)) = input.advance() else {
            return Err(format!("expected brackets but got: {:?}", input.peek()).into());
        };
        let tmp = Parse::parse(&mut brackets.tokens.clone().into())?;
        Ok(Self(brackets.span, tmp))
    }
}

#[derive(Debug, Clone)]
pub struct Braces<T>(pub Span, pub T);
impl<T> Parse for Braces<T>
where
    T: Parse,
{
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Some(Token::Brace(braces)) = input.advance() else {
            return Err("expected braces ".into())
        };
        let tmp = Parse::parse(&mut braces.tokens.clone().into())?;
        Ok(Self(braces.span, tmp))
    }
}
