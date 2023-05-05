use super::super::prelude::*;
#[derive(Debug, Clone)]
pub struct If {
    if_token: tokens::If,
    predicate: Box<Expression>,
    then_branch: Box<Statement>,
}
impl Parse for If {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let if_token = input.parse()?;
        let predicate = input.parse()?;
        let then_branch = input.parse()?;
        Ok(Self {
            if_token,
            predicate,
            then_branch,
        })
    }
}
