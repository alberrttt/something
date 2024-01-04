use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
pub fn parse_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    let data = match &input.data {
        Data::Struct(st) => st,
        _ => panic!("Only structs are supported"),
    };
    let fields = &data.fields;
    let field_names = fields.iter().map(|field| &field.ident);

    (quote! {
        impl<'a> Node<'a> for #name<'a> {
            fn parse(stream: &mut ParseStream<'a>) -> ParseResult<'a, Self> {
                Ok(
                    Self {
                        #(
                            #field_names: stream.parse()?,
                        )*
                    }
                )
            }
        }
    })
    .into()
}
