use proc_macro::TokenStream;
use quote::quote;
pub fn span_derive(parser: TokenStream) -> TokenStream {
    let derive_parser = syn::parse_macro_input!(parser as syn::DeriveInput);
    match derive_parser.data {
        syn::Data::Struct(_) => {
            let name = derive_parser.ident;
            quote! {
                impl #name {
                   pub fn span(&self) -> Span {
                        self.span.clone()
                    }
                }
            }
            .into()
        }
        syn::Data::Enum(e) => {
            let name = derive_parser.ident;
            let mut arms = Vec::new();
            for variant in e.variants {
                let variant_name = variant.ident;
                let fields = variant.fields;
                let mut field_names = Vec::new();
                for field in fields {
                    let field_name = field.ident;
                    field_names.push(field_name);
                }
                arms.push(quote! {
                    #name::#variant_name(field) => {
                       field.span
                    }
                });
            }
            quote! {
                impl #name {
                    pub fn span(&self) -> Span {
                        match self {
                            #(#arms),*
                        }
                    }
                }
            }
            .into()
        }
        syn::Data::Union(_) => todo!(),
    }
}
