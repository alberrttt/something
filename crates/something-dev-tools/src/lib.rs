use proc_macro::TokenStream;

mod tokens;
#[proc_macro]
pub fn tokens_ident(input: TokenStream) -> TokenStream {
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
#[proc_macro_attribute]
pub fn ast_test_gen(attr: TokenStream, item: TokenStream) -> TokenStream {
    ast_test_gen::ast_test_gen(attr, item)
}
mod tuple_parse_impl;
#[proc_macro]
pub fn tuple_parse_impl(input: TokenStream) -> TokenStream {
    tuple_parse_impl::tuple_parse_impl(input)
}
mod item_name;
#[proc_macro]
pub fn item_name(input: TokenStream) -> TokenStream {
    use quote::quote;

    quote! {}.into()
}
mod span_derive;
#[proc_macro_derive(Span)]
pub fn span_derive(input: TokenStream) -> TokenStream {
    span_derive::span_derive(input)
}
