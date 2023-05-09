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
mod parse_tokens_display;
#[proc_macro_derive(ParseTokensDisplay)]
pub fn parse_tokens_display(input: TokenStream) -> TokenStream {
    parse_tokens_display::parse_tokens_display(input)
}
mod ast_test_gen;
#[proc_macro]
pub fn ast_test_gen(input: TokenStream) -> TokenStream {
    ast_test_gen::ast_test_gen(input)
}
mod tuple_parse_impl;
#[proc_macro]
pub fn tuple_parse_impl(input: TokenStream) -> TokenStream {
    tuple_parse_impl::tuple_parse_impl(input)
}
