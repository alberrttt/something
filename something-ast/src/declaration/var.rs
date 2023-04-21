use something_frontend_tokenizer::{
    tokens::{self, Ident},
    Literal,
};

pub struct VariableDeclaration {
    pub let_token: tokens::Let,
    pub name: Ident,
    pub equal: tokens::Equal,
    pub value: Literal,
}
