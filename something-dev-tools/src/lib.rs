use proc_macro::TokenStream;

mod tokens;
#[proc_macro]
pub fn tokens_ident(parser: TokenStream) -> TokenStream {
    tokens::tokens(parser)
}
mod node;
#[proc_macro_derive(Node)]
pub fn node(parser: TokenStream) -> TokenStream {
    node::node(parser)
}
