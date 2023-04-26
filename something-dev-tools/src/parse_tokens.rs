use proc_macro::TokenStream;
use syn::{parse_macro_input, token::Token, Data, DeriveInput, Type};

use quote::quote;
pub fn parse_tokens(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);
    match derive.data {
        Data::Enum(enum_data) => {
            let name = derive.ident;

            let variants = enum_data.variants.iter().map(|f| {
                let variant_ident = &f.ident;
                quote! {
                    if let Ok(variant) = input.step(|input| Parse::parse(input)) {
                        return Ok(#name::#variant_ident(variant));
                    }
                }
            });
            return quote! {
                use something_frontend_tokenizer::Tokens;
                impl Parse for #name {
                    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
                        #(#variants)*
                        Err(format!("unexpected token(s) {}", input).into())
                    }
                }
            }
            .into();
        }
        Data::Struct(struct_data) => {
            let name = derive.ident;

            let variants = struct_data.fields.iter().map(|f| {
                let ident = f.ident.as_ref().expect("unnamed fields unsupported");
                if let Type::Tuple(typetuple) = &f.ty {
                    if typetuple.elems.is_empty() {
                        quote! {#ident: (),}
                    } else {
                        panic!("only empty types r supported")
                    }
                } else {
                    quote! {#ident: Parse::parse(input)?,}
                }
            });
            return quote! {
                use something_frontend_tokenizer::Tokens;
                impl Parse for #name {
                    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
                        Ok(Self {#(#variants)*})
                    }
                }
            }
            .into();
        }
        Data::Union(enum_data) => panic!("unions unsupported"),
    }
    panic!()
}
