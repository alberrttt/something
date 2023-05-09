use proc_macro::{Ident, TokenStream};
use quote::ToTokens;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Token};

// impl<$($ty),*> Parse for ($($ty),*)
// where
//     $($ty: Parse),*
// {
//     fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>> where Self: Sized {
//         Ok(($($ty::parse(input)?),*))
//     }
// }
// impl<$($ty),*> ParsingDisplay for ($($ty),*) where $($ty: ParsingDisplay),* {
//     fn display(&self) -> String where Self: Sized {
//         String::new()
//     }
//     fn placeholder() -> String where Self: Sized {
//         format!(concat!($(stringify!($ty), ": {} ",)*), $($ty::placeholder(),)*)
//     }
// }
struct P(Punctuated<syn::Ident, Token![,]>);
impl Parse for P {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_terminated(input)?))
    }
}
pub fn tuple_parse_impl(input: TokenStream) -> TokenStream {
    let idents = parse_macro_input!(input as P);
    let tokens = idents.0.iter().map(|f| f.to_owned()).collect::<Vec<_>>();
    let display = idents
        .0
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let ident = f.to_owned();
            let idx = syn::Index::from(i);
            quote::quote! {
                let #ident = self.#idx.display();
            }
        })
        .collect::<Vec<_>>();
    // format!("{A} {B} {C}")
    let format_string = idents
        .0
        .iter()
        .map(|f| {
            let ident = f.to_owned();
            quote::quote! {
                #ident
            }
        })
        .collect::<Vec<_>>();
    let display = quote::quote! {
        #(#display)*
        let mut s = String::new();
        #(s.push_str(#format_string.as_str());)*
        s
    };
    quote::quote! {
        impl<#(#tokens),*> Parse for (#(#tokens),*)
        where
        #(#tokens: Parse),*
        {
            fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>> where Self: Sized {
                Ok((#(#tokens::parse(input)?),*))
            }
        }
        impl<#(#tokens),*> ParsingDisplay for (#(#tokens),*)
        where
        #(#tokens: ParsingDisplay),*
        {
            fn display(&self) -> String where Self: Sized {
                #display
            }
            fn placeholder() -> String where Self: Sized {
                format!(concat!(#(stringify!(#tokens), ": {} ",)*), #(#tokens::placeholder(),)*)
            }
        }
    }
    .into()
}
