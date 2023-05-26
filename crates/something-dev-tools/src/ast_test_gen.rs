use proc_macro::TokenStream;


use syn::parse_macro_input;
pub fn ast_test_gen(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let _item = parse_macro_input!(input as syn::DeriveInput);
    todo!()
}
