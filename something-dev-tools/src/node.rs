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
    recover: proc_macro2::TokenStream,
    append_tokens: proc_macro2::TokenStream,
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
    let append_tokens = &gen.append_tokens;
    let tmp_name = format_ident!("tmp{}", name);
    let generic_params = derive_input.generics;
    let generic_as_part_of_types = {
        let params = generic_params.type_params().map(|param| {
            let ident = &param.ident;
            quote! {#ident}
        });
        let lt = generic_params.lt_token;
        let gt = generic_params.gt_token;
        quote! {
            #lt #( #params ),* #gt
        }
    };
    let recover_tokens = &gen.recover;
    let output = quote! {
        #[allow(non_snake_case)]
        mod #tmp_name {
            use crate::prelude::*;
            use super::#name;
            impl #generic_params Node for #name #generic_as_part_of_types{
                fn parse(parser: &mut Parser) -> ParseResult<Self>
                where
                    Self: Sized
                {
                    #parse
                }
                fn span(&self) -> Span {
                    #span
                }
                fn append_tokens(&self, to: &mut Vec<Token>) {
                    #append_tokens
                }
                fn recover(parser: &mut Parser)
                where
                    Self: Sized
                {
                    #recover_tokens
                }
            }
        }
        use #tmp_name::*;

    };

    output.into()
}
fn gen_struct(ident: Ident, data: &DataStruct) -> Gen {
    let mut fields = data.fields.iter();
    let first = fields.next().unwrap();
    use struct_gen::*;
    let parse = struct_gen_parse(&ident, data, &fields, first);
    let span = struct_gen_span(&ident, data, &fields, first);
    let append_tokens = struct_gen_append_tokens(&ident, data, &fields, first);
    let recover = struct_gen_recover(&ident, data, &fields, first);
    Gen {
        parse,
        recover,
        span,
        append_tokens,
        name: ident,
    }
}

mod struct_gen {
    use super::*;
    pub fn struct_gen_recover(
        ident: &Ident,
        data: &DataStruct,
        fields: &syn::punctuated::Iter<Field>,
        first_field: &Field,
    ) -> proc_macro2::TokenStream {
        let last_field = fields.clone().last().unwrap_or(first_field);
        let last_ident = last_field.ident.as_ref().unwrap();
        let last_ty = &last_field.ty;
        let first_ident = first_field.ident.as_ref().unwrap();
        let first_ty = &first_field.ty;
        quote! {
            // consume all tokens until it reaches the last field

        }
    }
    pub fn struct_gen_span(
        ident: &Ident,
        data: &DataStruct,
        fields: &syn::punctuated::Iter<Field>,
        first: &Field,
    ) -> proc_macro2::TokenStream {
        let last = fields.clone().last().unwrap_or(first);
        let last_ident = last.ident.as_ref().unwrap();
        let first_ident = first.ident.as_ref().unwrap();
        quote! {
            Span {
                start: self.#first_ident.span().start,
                end: self.#first_ident.span().end,
                line: self.#first_ident.span().line,
                line_start: self.#last_ident.span().line_start,
            }
        }
    }
    pub fn struct_gen_append_tokens(
        ident: &Ident,
        data: &DataStruct,
        fields: &syn::punctuated::Iter<Field>,
        first: &Field,
    ) -> proc_macro2::TokenStream {
        let lines = fields.clone().map(|field| {
            let name = &field.ident;
            quote! {
                self.#name.append_tokens(to);
            }
        });
        let first_ident = first.ident.as_ref().unwrap();
        quote! {
            self.#first_ident.append_tokens(to);
            #( #lines )*
        }
    }

    pub fn struct_gen_parse(
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
                    let #name = match parser.step(|f| <#ty as Node>::parse(f)) {
                        Ok(v) => v,
                        Err(e) => {
                            <#ty as Node>::recover(parser);
                            return Err(e);
                        }
                    };
                }
            })
            .collect();
        let final_line = {
            let name = &first.ident;
            let fields = fields.clone().map(|field| &field.ident);

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
                let Ok(first) = <#ty as Node>::parse(parser) else {
                    panic!("parse error")
                };
                #( #parse_lines )*
                #final_line
            }
        };
        parse
    }
}
