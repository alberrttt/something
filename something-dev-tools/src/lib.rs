use proc_macro::TokenStream;
use syn::token::Token;

mod tokens;
#[proc_macro]
pub fn tokens(input: TokenStream) -> TokenStream {
    tokens::tokens(input)
}
mod parse_tokens;

#[proc_macro_derive(ParseTokens)]
pub fn parse_tokens(input: TokenStream) -> TokenStream {
    parse_tokens::parse_tokens(input)
}
