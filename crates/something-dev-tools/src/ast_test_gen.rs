use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;
pub fn ast_test_gen(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::DeriveInput);
    todo!()
}
