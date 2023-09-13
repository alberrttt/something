use proc_macro::TokenStream;

mod tokens;
#[proc_macro]
pub fn tokens_ident(parser: TokenStream) -> TokenStream {
    tokens::tokens(parser)
}
