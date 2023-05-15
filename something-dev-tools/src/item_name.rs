use syn::parse::Parse;
use syn::token::Token;
use syn::Token;
pub struct IdentName(pub syn::Ident, pub Token![,], pub syn::LitStr);
impl Parse for IdentName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?, input.parse()?, input.parse()?))
    }
}
pub fn item_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ident_name = syn::parse_macro_input!(input as IdentName);
    let ident = &ident_name.0;
    let name = &ident_name.2;
    quote::quote! {
        impl #ident {
            pub fn name() -> &'static str {
                #name
            }
        }
    }
    .into()
}
