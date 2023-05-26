use quote::format_ident;
use syn::parse::Parse;

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
    let fmtident = format_ident!("__{}", ident);
    quote::quote! {

        mod #fmtident {
            use super::#ident;
            use crate::prelude::*;
            impl Name for #ident {
                 fn name() -> &'static str {
                    #name
                }
                 fn named(&self) -> &'static str {
                    #name
                }
            }
        }
        pub use #fmtident::*;
    }
    .into()
}
