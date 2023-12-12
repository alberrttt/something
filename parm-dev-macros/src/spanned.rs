use proc_macro::TokenStream;

use quote::{quote};
use syn::{
    parse_macro_input, Data, DeriveInput,
};
pub fn spanned_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut impls = quote! {};
    match input.data {
        Data::Struct(data) => {
            let fields = data.fields.iter().collect::<Vec<_>>();
            let first_field = fields.first().unwrap();
            let last_field = fields.last().unwrap();

            let first_ident = first_field.ident.as_ref().unwrap();
            let last_ident = last_field.ident.as_ref().unwrap();

            let first_field_ty = &first_field.ty;
            let last_field_ty = &last_field.ty;
            let first_is_option = match first_field_ty {
                syn::Type::Path(path) => {
                    let path = &path.path;
                    let segments = &path.segments;
                    let first_segment = segments.first().unwrap();
                    let first_segment_ident = &first_segment.ident;

                    first_segment_ident == "Option"
                }
                _ => false,
            };
            let last_is_option = match last_field_ty {
                syn::Type::Path(path) => {
                    let path = &path.path;
                    let segments = &path.segments;
                    let first_segment = segments.first().unwrap();
                    let first_segment_ident = &first_segment.ident;
                    first_segment_ident == "Option"
                }
                _ => false,
            };

            if first_is_option && last_is_option {
                impls = quote! {
                    match (self.#first_ident.as_ref(), self.#last_ident.as_ref()) {
                        (Some(first), Some(last)) => first.span().join(last.span()),
                        (Some(first), None) => first.span(),
                        (None, Some(last)) => last.span(),
                        (None, None) => panic!(),
                    }
                };
            } else if first_is_option {
                impls = quote! {
                    match self.#first_ident.as_ref() {
                        Some(inner) => inner.span(),
                        None => self.#last_ident.span(),
                    }.join(self.#last_ident.span())

                };
            } else if last_is_option {
                impls = quote! {
                    match self.#last_ident.as_ref() {
                        Some(inner) => self.#first_ident.span().join(inner.span()),
                        None => self.#first_ident.span(),
                    }
                };
            } else {
                impls = quote! {
                    self.#first_ident.span().join(self.#last_ident.span())
                };
            }
        }
        Data::Enum(data) => {
            let mut arms = Vec::new();
            for variant in data.variants.iter() {
                let ident = &variant.ident;
                arms.push(quote! {
                    Self::#ident(inner) => inner.span(),
                });
            }

            impls = quote! {
                match self {
                    #(#arms)*
                }
            };
        }
        _ => panic!("This macro only supports structs with named fields."),
    }
    let mut generics = quote! {};
    for param in input.generics.params.iter() {
        match param {
            syn::GenericParam::Lifetime(_) => {
                generics = quote! {#generics '_,};
            }
            syn::GenericParam::Type(_) => generics = quote! {#generics _,},
            syn::GenericParam::Const(_) => todo!(),
        }
    }

    let expanded = quote! {
        impl Spanned for #name<#generics> {
            fn span(&self) -> parm_common::Span {
                #impls
            }
        }
    };

    expanded.into()
}
