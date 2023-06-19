use proc_macro::TokenStream;

use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Token};

// impl<$($ty),*> Parse for ($($ty),*)
// where
//     $($ty: Parse),*
// {
//     fn parse(parser: &mut Tokens) -> Result<Self, ParseError> where Self: Sized {
//         Ok(($($ty::parse(parser)?),*))
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
    fn parse(parser: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_terminated(parser)?))
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
    let display_option = {
        let display = idents
            .0
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let ident = f.to_owned();
                let idx = syn::Index::from(i);
                quote::quote! {
                    let #ident = match self.as_ref() {
                        Some(s) => s.#idx.display(),
                        None => String::new(),
                    };
                }
            })
            .collect::<Vec<_>>();
        quote::quote! {
            #(#display)*
            let mut s = String::new();
            #(s.push_str(#format_string.as_str());)*
            s
        }
    };
    let tokens_parsing = tokens.iter().skip(1).collect::<Vec<_>>();
    quote::quote! {
        impl<#(#tokens),*> AppendTokens for (#(#tokens),*)  where
        #(#tokens: AppendTokens),*
        {
            fn append_tokens(&self, tokens: &mut TokenStream) where Self: Sized {
                let (#(#tokens),*) = self;
                #(#tokens.append_tokens(tokens);)*
            }
        }

        impl<#(#tokens),*> Parse for (#(#tokens),*)
        where
        #(#tokens: Parse),*
        {
            fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> where Self: Sized {
                let tmp =  match A::parse(parser) {
                    Ok(ok) => ok,
                    Err(_) | Recoverable => return Recoverable,
                };

                Ok((tmp, #(#tokens_parsing::parse(parser).unwrap()),*))
            }
        }
        impl<#(#tokens),*> Parse for Option<(#(#tokens),*)>
        where
        #(#tokens: Parse),*
        {
            fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> where Self: Sized {
                let tmp = match parser.step(|parser| match A::parse(parser) {
                    Ok(ok) => Ok(ok),
                    Err(_) | Recoverable => return Recoverable,
                }) {
                    Ok(ok) => ok,
                    Err(_) | Recoverable => return Ok(None),
                };

                Ok(Some((tmp, #(#tokens_parsing::parse(parser).unwrap()),*)))
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
                format!("{}", concat!(#(concat!(stringify!(#tokens), "")),*))
            }
        }
        impl<#(#tokens),*> ParsingDisplay for Option< (#(#tokens),*)>
        where
        #(#tokens: ParsingDisplay),*
        {
            fn display(&self) -> String where Self: Sized {
                #display_option
            }
            fn placeholder() -> String where Self: Sized {
                format!("{}", concat!(#(concat!(stringify!(#tokens), "")),*))
            }
        }
    }
    .into()
}
