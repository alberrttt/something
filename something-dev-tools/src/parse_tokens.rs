use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, token::Token, Data, DeriveInput, Type};
pub fn parse_tokens(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);
    match derive.data {
        Data::Enum(enum_data) => {
            let name = derive.ident;

            let variants = enum_data.variants.iter().map(|f| {
                let variant_ident = &f.ident;
                quote! {

                    match input.step(|input| Parse::parse(input)) {
                        Ok(variant) => return Ok(#name::#variant_ident(variant)),
                        Err(x) => err = x,
                    }
                }
            });

            return quote! {
                impl Parse for #name {
                    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
                        let mut err: Box<dyn std::error::Error> = "".into();
                        #(#variants)*
                        Err(err)
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
