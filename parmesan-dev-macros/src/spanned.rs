use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Error, ExprParen, ExprPath,
    Fields, Path,
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

            impls = quote! {

                (self.#first_ident.span(),self.#last_ident.span()).into()
            };
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
            fn span(&self) -> parmesan_common::Span {
                #impls
            }
        }
    };

    expanded.into()
}
