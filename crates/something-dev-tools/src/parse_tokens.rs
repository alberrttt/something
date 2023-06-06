use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};

use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Type};
pub fn parse_tokens(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);

    match derive.data {
        Data::Enum(enum_data) => {
            let name = derive.ident;

            let variants = enum_data.variants.iter().enumerate().map(|(i,f)| {
                let variant_ident = &f.ident;
                let fields = &f.fields;
                match fields {
                    syn::Fields::Named(_) => todo!(),
                    syn::Fields::Unnamed(fields) => {
                        let fields = &fields.unnamed;
                        let create = fields
                            .iter()
                            .skip(1)
                            .map(|_| {
                                quote! {input.step(|input| Parse::parse(input)).unwrap()}
                            })
                            .collect::<Vec<_>>();
                        let err_str = if i == enum_data.variants.len() - 1 {
                            quote! {
                                concat!("or ", stringify!(#variant_ident))
                            }
                        } else {
                            quote! {
                                concat!(stringify!(#variant_ident), ", ")
                            }
                        };
                        quote! {
                            match input.step(|input| Parse::parse(input)) {
                                Ok(variant) => return Ok(#name::#variant_ident(variant, #(#create),*)),
                                Err(x) => {

                                    err.push_str(#err_str);
                                },
                            }
                        }
                    }
                    syn::Fields::Unit => todo!(),
                }
            });

            let ident = format_ident!("__{}", name.to_string().to_lowercase());

            return quote! {
                mod #ident {
                    use colored::Colorize;
                    use something_frontend_tokenizer::prelude::*;
             
                    use std::fmt::{Display, Formatter};
                    use super::#name;
                    impl Parse for #name {
                        fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
                            let mut err = String::from("Expected ").yellow().to_string();
                            #(#variants)*
                            err.push_str(format!("\n{} {}","But got:".red(), input.peek().unwrap()).as_str());
                            Err(ParseError::Generic(err))
                        }
                    }

                    impl Parse for Box<#name> {
                        fn parse(input: &mut Tokens) -> Result<Self,ParseError> {
                            Ok(Box::new(#name::parse(input)?))
                        }
                    }
                }
                pub use #ident::*;

            }
            .into();
        }
        Data::Struct(struct_data) => {
            let name = derive.ident;
            let parse_impl = if struct_data.fields.iter().any(|f| f.ident.is_some()) {
                for_struct_w_named_fields(&struct_data, &name)
            } else {
                for_struct_w_unamed_fields(&struct_data, &name)
            };
            let ident = format_ident!("__{}", name.to_string().to_lowercase());
            let to_vec: Vec<proc_macro2::TokenStream >= {
                struct_data.fields.iter().map(|f| {
                    let ident = f.ident.as_ref().expect("unnamed fields unsupported");
                    quote! {self.#ident.clone().append_tokens(tokens);}
                })
            }.collect();
            return quote! {
                mod #ident {
                    use colored::Colorize;
                    use something_frontend_tokenizer::prelude::*;
                    use std::fmt::{Display, Formatter};
                    use super::#name;
                    #parse_impl
                    impl AppendTokens for #name {
                        fn append_tokens(&self, tokens: &mut Tokens) {
                            #(#to_vec)*
                        }
                    }
                    impl Parse for Box<#name> {
                        fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
                            Ok(Box::new(#name::parse(input)?))
                        }
                    }
                }
                pub use #ident::*;

            }
            .into();
        }
        Data::Union(_enum_data) => panic!("unions unsupported"),
    }
    panic!()
}
fn for_struct_w_named_fields(struct_data: &DataStruct, name: &Ident) -> proc_macro2::TokenStream {
    let mut iter = struct_data.fields.iter();
    let variant = iter.next().unwrap();
    let variants = iter.map(|f| {
        let ident = f.ident.as_ref().expect("unnamed fields unsupported");
        dbg!(ident);
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
    let variant_identifier = &variant.ident;
    quote! {

            impl Parse for #name {
                fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
                    let tmp = input.step(|input| Parse::parse(input));
                    match tmp {
                        Ok(tmp) => {
                            Ok(Self {
                                #variant_identifier: tmp,
                                #(#variants)*
                            })
                        }
                        Err(err) => Err(err),
                    }
                }
            }

    }
}
fn for_struct_w_unamed_fields(struct_data: &DataStruct, name: &Ident) -> proc_macro2::TokenStream {
    let fields = struct_data.fields.iter().skip(1).map(|_field| {
        quote! {
            Parse::parse(input).unwrap()
        }
    });
    quote! {


            impl Parse for #name {
                fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
                    let tmp = input.step(|input| Parse::parse(input));
                    match tmp {
                        Ok(tmp) => {
                            Ok(Self(tmp, #(#fields),*))
                        }
                        Err(err) => Err(err),
                    }
                }
            }

    }
}
