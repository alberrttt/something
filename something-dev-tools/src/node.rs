use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::token::{Final, Struct, Token};
use syn::{parse_macro_input, DataStruct};
use syn::{DeriveInput, Field};
#[derive(Debug, Clone)]
struct Gen {
    span: proc_macro2::TokenStream,
    parse: proc_macro2::TokenStream,
    into_tokens: proc_macro2::TokenStream,
    name: syn::Ident,
}
pub fn node(stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(stream as DeriveInput);

    let gen: Gen = match &derive_input.data {
        syn::Data::Struct(data) => gen_struct(derive_input.ident, data),
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };
    let name = &gen.name;
    let parse = &gen.parse;
    let span = &gen.span;
    let into_tokens = &gen.into_tokens;
    let tmp_name = format_ident!("tmp{}", name);
    let output = quote! {
        mod #tmp_name {
            use crate::prelude::*;

            impl Node for #name {
                fn parse(parser: &mut Parser) -> ParseResult<Self>
                where
                    Self: Sized
                {
                    #parse
                }
                fn span(&self) -> Span {
                    #span
                }
                fn into_tokens(&self) -> Vec<Token> {
                    #into_tokens
                }
            }
        }
        use tmp_name::*;

    };

    output.into()
}
fn gen_struct(ident: Ident, data: &DataStruct) -> Gen {
    let mut fields = data.fields.iter();
    let first = fields.next().unwrap();
    let parse = struct_gen_parse(&ident, data, &fields, first);
    let span = struct_gen_span(&ident, data, &fields, first);
    let into_tokens = struct_gen_into_tokens(&ident, data, &fields, first);
    Gen {
        parse,

        span,
        into_tokens,
        name: ident,
    }
}
fn struct_gen_span(
    ident: &Ident,
    data: &DataStruct,
    fields: &syn::punctuated::Iter<Field>,
    first: &Field,
) -> proc_macro2::TokenStream {
    quote! {}
}
fn struct_gen_into_tokens(
    ident: &Ident,
    data: &DataStruct,
    fields: &syn::punctuated::Iter<Field>,
    first: &Field,
) -> proc_macro2::TokenStream {
    quote! {}
}

fn struct_gen_parse(
    ident: &Ident,
    data: &DataStruct,
    fields: &syn::punctuated::Iter<Field>,
    first: &Field,
) -> proc_macro2::TokenStream {
    let parse_lines: Vec<proc_macro2::TokenStream> = fields
        .clone()
        .map(|field| {
            let name = &field.ident;
            let ty = &field.ty;
            quote! {
                let #name = <#ty as Node>::parse()?;
            }
        })
        .collect();
    let final_line = {
        let name = &first.ident;
        let fields = data.fields.iter().map(|field| &field.ident);
        let ty = &first.ty;
        quote! {
            Ok(Self {
                #name: first,
                #( #fields ),*
            })
        }
    };

    let parse = {
        let ty = &first.ty;
        quote! {
            let Ok(first) = <#ty as Node>::parse() else {
                panic!("parse error")
            };
            #( #parse_lines )*
            #final_line
        }
    };
    parse
}
