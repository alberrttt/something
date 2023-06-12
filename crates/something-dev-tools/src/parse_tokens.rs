use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};

use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Type};
pub fn parse_tokens(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);

    match derive.data {
        Data::Enum(enum_data) => {
            let name = derive.ident;

            let variants = enum_data.variants.iter().enumerate().map(|(i, f)| {
                let variant_ident = &f.ident;
                let fields = &f.fields;
                match fields {
                    syn::Fields::Named(_) => todo!(),
                    syn::Fields::Unnamed(_) => {
                        quote! {
                            match input.step(|input| Parse::parse(input)) {
                                Ok(variant) => return Ok(#name::#variant_ident(variant)),

                                Err(err) => {
                                    return Err(err);
                                }

                                Recoverable => {
                                }
                                // since we are parsing the first field/node (or token)
                                // we CAN recover from this error
                                // because other nodes might be valid because of the first node
                                // but if we are parsing the second node (or any constituent node)
                                // we can't recover from this error
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
                    use crate::tokenizer::prelude::*;
                    use crate::prelude::*;
                    use std::fmt::{Display, Formatter};
                    use super::#name;
                    impl Parse for #name {
                        fn parse(input: &mut Tokens) -> ParseResult<Self> {
                            #(#variants)*
                            Recoverable
                        }
                    }

                    impl Parse for Box<#name> {
                        fn parse(input: &mut Tokens) -> ParseResult<Self> {
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
            let to_vec: Vec<proc_macro2::TokenStream> = {
                struct_data.fields.iter().map(|f| {
                    let ident = f.ident.as_ref().expect("unnamed fields unsupported");
                    quote! {self.#ident.clone().append_tokens(tokens);}
                })
            }
            .collect();
            return quote! {
                mod #ident {
                    use colored::Colorize;
                    use crate::tokenizer::prelude::*;
                    use crate::prelude::*;
                    use std::fmt::{Display, Formatter};
                    use super::#name;
                    #parse_impl
                    impl AppendTokens for #name {
                        fn append_tokens(&self, tokens: &mut Tokens) {
                            #(#to_vec)*
                        }
                    }
                    impl Parse for Box<#name> {
                        fn parse(input: &mut Tokens) -> ParseResult<Self> {
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
                fn parse(input: &mut Tokens) -> ParseResult<Self> {
                    let tmp = input.step(|input| Parse::parse(input));
                    match tmp {
                        Ok(tmp) => {
                            Ok(Self {
                                #variant_identifier: tmp,
                                #(#variants)*
                            })
                        }
                        Err(err) => Err(err),
                        Recoverable => {
                            Recoverable
                        }
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
                fn parse(input: &mut Tokens) -> ParseResult<Self> {
                    let tmp = input.step(|input| Parse::parse(input));
                    match tmp {
                        Ok(tmp) => {
                            Ok(Self(tmp, #(#fields),*))
                        }
                        Err(err) => Err(err),
                        Recoverable => {
                            Recoverable
                        }
                    }
                }
            }

    }
}
