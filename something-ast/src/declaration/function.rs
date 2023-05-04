use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{
    ident::Ident,
    tokens::{self, Colon, Parse, Token},
    Token, Tokens,
};

use crate::{
    delimiter::{Braces, Brackets, Parenthesis},
    expression::Expression,
    punctuated::Punctuated,
    Statement, Node,
};

#[derive(Debug, ParseTokens, Clone)]
pub struct FunctionDeclaration {
    pub modifiers: Brackets<Vec<Ident>>,
    pub colon: Colon,
    pub fn_token: tokens::Fn,
    pub name: Ident,
    pub params: Parenthesis<Ident>,   // todo
    pub body: Braces<Vec<Node>>, // todo
}

