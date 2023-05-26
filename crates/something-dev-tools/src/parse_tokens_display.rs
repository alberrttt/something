use proc_macro::TokenStream;

use quote::{format_ident, TokenStreamExt};
use syn::DeriveInput;
pub fn parse_tokens_display(input: TokenStream) -> TokenStream {
    let derive = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &derive.ident;

    let ident = format_ident!("__{}__display", name.to_string().to_lowercase());
    let display = calculate_display(&derive);
    let expanded = quote::quote! {
        mod #ident {
            use super::#name;
            use something_frontend_tokenizer::traits::ParsingDisplay;
            impl ParsingDisplay for #name {
               fn display(&self) -> String
               where
                   Self: Sized,
               {
                let mut s = String::new();
                #display
                s
               }
               fn placeholder() -> String
               where
                   Self: Sized,
               {
                   format!("<{}>", stringify!(#name))
               }
            }

        }
    };

    expanded.into()
}
pub fn calculate_display(derive: &DeriveInput) -> proc_macro2::TokenStream {
    match &derive.data {
        syn::Data::Struct(s) => {
            let mut stream = quote::quote! {
                use std::fmt::Write;
            };

            s.fields.iter().enumerate().for_each(|(i, f)| {
                if let Some(name) = &f.ident {
                    stream.append_all(quote::quote! {
                        write!(s, "{} ", &self.#name.display()).unwrap();
                    })
                } else {
                    let index = syn::Index::from(i);
                    stream.append_all(quote::quote! {
                        write!(s, "{} ", &self.#index.display()).unwrap();
                    })
                }
            });
            stream
        }
        syn::Data::Enum(e) => {
            let e_variants_display = e
                .variants
                .iter()
                .map(|f| {
                    let name = &f.ident;
                    let tmp = f
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, _f)| {
                            let ident = format_ident!("__{}", i);
                            quote::quote! {
                                write!(s, "{} ", #ident.display()).unwrap();
                            }
                        })
                        .collect::<Vec<_>>();
                    let fields = f
                        .fields
                        .iter()
                        .enumerate()
                        .map(|f| {
                            let tmp = format_ident!("__{}", f.0);
                            quote::quote! {
                                #tmp
                            }
                        })
                        .collect::<Vec<_>>();

                    quote::quote! {
                        Self::#name(#(#fields),*) => {
                            #(#tmp)*
                        }
                    }
                })
                .collect::<Vec<_>>();
            quote::quote! {
                use std::fmt::Write;
                match self {
                    #(#e_variants_display)*
                }
            }
        }
        syn::Data::Union(_) => todo!(),
    }
}
