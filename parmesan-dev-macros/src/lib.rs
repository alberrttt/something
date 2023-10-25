use std::{collections::HashMap, path::Path};

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use spanned::spanned_derive;
use syn::{
    parse::{Parse, ParseBuffer},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Comma, Token},
    Attribute, Expr, ExprLit, Lit, LitStr, Meta,
};

#[proc_macro]
pub fn lower_stringify(input: TokenStream) -> TokenStream {
    // Parse the input token stream as a string literal
    let input_str = parse_macro_input!(input as Ident);

    // Get the string value from the literal and convert it to lowercase
    let input_string = input_str.to_string().to_lowercase();

    // Create a new string literal with the lowercase value
    let lower_literal = LitStr::new(&input_string, input_str.span());

    // Generate the final token stream
    let result = quote! {
        #lower_literal
    };

    result.into()
}

mod spanned;
#[proc_macro_derive(Spanned)]
pub fn derive_spanned(input: TokenStream) -> TokenStream {
    spanned_derive(input)
}
mod gen_token;
#[proc_macro]
pub fn gen_token(input: TokenStream) -> TokenStream {
    gen_token::gen_token(input)
}
mod parse_derive;
#[proc_macro_derive(Parse)]
pub fn parse(input: TokenStream) -> TokenStream {
    parse_derive::parse_derive(input)
}
