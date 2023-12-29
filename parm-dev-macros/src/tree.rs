use quote::quote;
use syn::{parse_macro_input, Label, LitStr};
pub fn tree(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident;
    let name_str_lit = LitStr::new(&name.to_string(), name.span());
    let mut tree_impl: proc_macro2::TokenStream = quote! {};
    match input.data {
        syn::Data::Struct(data) => {
            let mut methods: Vec<proc_macro2::TokenStream> = Vec::new();
            for field in data.fields.iter() {
                let name = field.ident.as_ref().unwrap();
                let label = LitStr::new(&name.to_string(), name.span());
                methods.push(quote! {
                    .child(self.#name.tree().label(#label))
                });
            }
            tree_impl = quote! {
                Tree::new(#name_str_lit)#(#methods)*
            };
        }
        syn::Data::Enum(data) => {
            let mut methods: Vec<proc_macro2::TokenStream> = Vec::new();
            for variant in data.variants.iter() {
                let variant_name = &variant.ident;
                let variant_name_str_lit =
                    LitStr::new(&variant_name.to_string(), variant_name.span());
                let mut variant_methods: Vec<proc_macro2::TokenStream> = Vec::new();

                methods.push(quote! {
                    #name::#variant_name( t ) => t.tree()
                });
            }
            tree_impl = quote! {
                match self {
                    #(#methods),*
                }
            };
        }
        syn::Data::Union(_) => todo!(),
    }
    quote! {
        impl TreeDisplay for #name<'_> {
            fn tree(&self) -> Tree {
                #tree_impl
            }
        }
    }
    .into()
}
