use quote::format_ident;
use syn::parse::Parse;

use syn::Token;
pub struct IdentName(pub syn::Ident, pub Token![,], pub syn::LitStr);
impl Parse for IdentName {
    fn parse(parser: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(parser.parse()?, parser.parse()?, parser.parse()?))
    }
}
pub fn item_name(parser: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ident_name = syn::parse_macro_input!(parser as IdentName);
    let ident = &ident_name.0;
    let name = &ident_name.2;
    let fmtident = format_ident!("__{}", ident);
    quote::quote! {

        mod #fmtident {
            use crate::tokenizer::prelude::Name;
            use super::#ident;
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
