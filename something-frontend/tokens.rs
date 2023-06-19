use std::fmt::Display;

use proc_macro::{token_stream, TokenStream};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Arm, Ident, Token};
struct Idents(pub Punctuated<Ident, Token![,]>);
impl Parse for Idents {
    fn parse(parser: syn::parse::ParseStream) -> syn::Result<Self> {
        let idents: Punctuated<Ident, Token![,]> = Punctuated::parse_terminated(parser)?;
        Ok(Self(idents))
    }
}
impl Display for Idents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ident in &self.0 {
            writeln!(f, "{}", ident)?;
        }
        Ok(())
    }
}
pub fn tokens(parser: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(parser as Idents);
    let arms = tokens
        .0
        .iter()
        .map(|ident| {
            let str_str = ident.to_string().to_lowercase();
            quote! {
                #str_str => Token!(self, #ident),
            }
        })
        .collect::<Vec<_>>();

    quote! {
        match ident.name.as_str() {
            #(#arms)*
            _ => Token::Ident(ident)
        }
    }
    .into()
}
