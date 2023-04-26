use something_frontend_tokenizer::{
    ident::Ident,
    lit::Literal,
    tokens::{self, Parse},
    Token,
};
#[derive(Debug)]
pub struct VariableDeclaration {
    pub let_token: tokens::Let,
    pub name: Ident,
    pub equal: tokens::Equal,
    pub value: Literal,
}
impl Parse for VariableDeclaration {
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            let_token: Parse::parse(input)?,
            name: Parse::parse(input)?,
            equal: Parse::parse(input)?,
            value: Parse::parse(input)?,
        })
    }
}
