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
pub struct Brackets<T>(pub Span, pub Vec<T>);
impl<T> Parse for Brackets<T>
where
    T: Parse,
{
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Some(Token::Bracket(brackets)) = input.advance() else {
            return Err(format!("Expected Brackets, got {:?} ", input.advance().clone()).into())
        };
        let mut parsed = Vec::new();
        let mut inner = Tokens::from(brackets.tokens.clone());
        loop {
            match Parse::parse(&mut inner) {
                Ok(parsed_t) => parsed.push(parsed_t),
                Err(v) => {
                    println!("{}", v);
                    break;
                }
            }
        }
        Ok(Self(brackets.span, parsed))
    }
}

#[derive(Debug, Clone)]
pub struct Braces<T>(pub Span, pub Vec<T>);
impl<T> Parse for Braces<T>
where
    T: Parse,
{
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Some(Token::Brace(braces)) = input.advance() else {
            panic!()
        };
        let mut parsed = Vec::new();
        let mut inner = Tokens::from(braces.tokens.clone());
        loop {
            match Parse::parse(&mut inner) {
                Ok(parsed_t) => parsed.push(parsed_t),
                Err(v) => {
                    println!("for braces: {}", v);
                    break;
                }
            }
        }
        Ok(Self(braces.span, parsed))
    }
}
