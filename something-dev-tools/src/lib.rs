use proc_macro::TokenStream;

mod tokens;
#[proc_macro]
pub fn tokens_ident(parser: TokenStream) -> TokenStream {
    tokens::tokens(parser)
}
mod parse_tokens;

#[proc_macro_derive(ParseTokens)]
pub fn parse_tokens(parser: TokenStream) -> TokenStream {
    parse_tokens::parse_tokens(parser)
}
mod parse_tokens_display;
#[proc_macro_derive(ParseTokensDisplay)]
pub fn parse_tokens_display(parser: TokenStream) -> TokenStream {
    parse_tokens_display::parse_tokens_display(parser)
}
mod ast_test_gen;
#[proc_macro_attribute]
pub fn ast_test_gen(attr: TokenStream, item: TokenStream) -> TokenStream {
    ast_test_gen::ast_test_gen(attr, item)
}
mod tuple_parse_impl;
#[proc_macro]
pub fn tuple_parse_impl(parser: TokenStream) -> TokenStream {
    tuple_parse_impl::tuple_parse_impl(parser)
}
mod item_name;
#[proc_macro]
pub fn item_name(_parser: TokenStream) -> TokenStream {
    use quote::quote;

    quote! {}.into()
}
mod span_derive;
#[proc_macro_derive(Span)]
pub fn span_derive(parser: TokenStream) -> TokenStream {
    span_derive::span_derive(parser)
}
