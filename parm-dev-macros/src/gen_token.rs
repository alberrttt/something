use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma, Attribute, Expr, Ident,
    Lit, LitStr, Meta,
};
struct Item(Option<Vec<Attribute>>, Ident);
impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(
            Attribute::parse_outer(input).ok(),
            Ident::parse(input)?,
        ))
    }
}
use quote::{format_ident, quote};
struct List(Punctuated<Item, Comma>);
impl Parse for List {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_terminated(input)?))
    }
}
struct Idents(Punctuated<Ident, Comma>);
impl Parse for Idents {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_terminated(input)?))
    }
}
#[derive(Debug, Clone)]
struct LexArm {
    char: char,
    children: HashMap<char, LexArm>,
    ident: Ident,
}
struct StructDefsTokenItems {
    pub token_items: Vec<TokenStream2>,
    pub struct_defs: Vec<TokenStream2>,
    pub groups: HashMap<String, Vec<Ident>>,
    pub lexemes: HashMap<char, (LexArm, LitStr)>,
}
fn generate_struct_defs_token_items(punctuation: &List) -> StructDefsTokenItems {
    let mut token_items = vec![];
    let mut struct_defs = vec![];
    let mut groups: HashMap<String, Vec<Ident>> = HashMap::new();
    let mut lexemes: HashMap<char, (LexArm, LitStr)> = HashMap::new();

    for item in punctuation.0.iter() {
        let ident = &item.1;
        token_items.push(quote! {
            #ident(#ident<'a>),
        });
        struct_defs.push(quote! {
            #[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Default)]
            pub struct #ident<'a> {
                pub lexeme: &'a str,
                pub span: Span
            }
            impl<'a> #ident<'a> {
                // Spanless Equal
                fn seq(&self, other: &#ident) -> bool {
                    self.lexeme == other.lexeme
                }
            }
            impl<'a> Spanned for #ident<'a> {
                fn span(&self) -> Span {
                    self.span
                }
            }
            impl<'a> Node<'a> for #ident<'a> {
                fn parse(
                    parser: &mut crate::parser::parse_stream::ParseStream<'a>,
                ) -> Result<Self, crate::error::ParseError<'a>>
                where
                    Self: Sized,
                {
                    let peeked = parser.peek()?;
                    if let Token::#ident(peeked) = peeked {
                        let tmp = Ok(peeked.clone());
                        parser.advance()?;
                        tmp
                    } else {
                        Err(
                            ParseError::ExpectedNode(
                                ExpectedNode {
                                    got: peeked.lexeme(),
                                    expected: stringify!(#ident)
                                }
                            )
                        )
                    }
                }
            }
            impl <'a> From<#ident<'a>> for Token<'a> {
                fn from(value: #ident<'a>) -> Token<'a> {
                    Token::#ident(value)
                }
            }
        });
        let attributes = match &item.0 {
            Some(some) => some,
            None => continue,
        };
        for attribute in attributes {
            let path = attribute.path();
            if path.is_ident("group") {
                if let Meta::List(list) = &attribute.meta {
                    let tokens: TokenStream = list.tokens.clone().into();
                    let group_idents = syn::parse::<Idents>(tokens).unwrap();
                    for group_ident in group_idents.0 {
                        groups
                            .entry(group_ident.to_string())
                            .or_default()
                            .push(ident.clone())
                    }
                }
            }
            if path.is_ident("lexeme") {
                if let Meta::NameValue(name_value) = &attribute.meta {
                    if let Expr::Lit(lit) = &name_value.value {
                        if let Lit::Str(str) = &lit.lit {
                            let lexeme = str.value();
                            let mut chars = lexeme.chars();

                            let first_char = chars.next().unwrap();
                            let tmp = lexemes.entry(first_char).or_insert((
                                LexArm {
                                    char: first_char,
                                    children: HashMap::new(),
                                    ident: ident.clone(),
                                },
                                str.clone(),
                            ));
                            let mut prev_arm = &mut tmp.0;
                            for char in chars {
                                let mut arm = prev_arm.children.entry(char).or_insert(LexArm {
                                    char,
                                    children: HashMap::new(),
                                    ident: ident.clone(),
                                });
                                prev_arm = arm;
                            }
                        }
                    }
                }
            }
        }
    }
    StructDefsTokenItems {
        token_items,
        struct_defs,
        groups,
        lexemes,
    }
}
pub fn gen_token(input: TokenStream) -> TokenStream {
    // input is `Ident,Ident,Ident`
    let punctuation = parse_macro_input!(input as List);

    let StructDefsTokenItems {
        token_items,
        struct_defs,
        groups,
        lexemes,
    } = generate_struct_defs_token_items(&punctuation);

    let groups = groups
        .iter()
        .map(|(group_name, items)| {
            let ident = format_ident!("{}", group_name);
            let members = items
                .iter()
                .map(|ident| quote! {#ident(#ident<'a>),})
                .collect::<Vec<_>>();
            let lexeme_arms = items
                .iter()
                .map(|member| quote! {#ident::#member(token) => token.lexeme,})
                .collect::<Vec<_>>();
            let spanned_arms = items
                .iter()
                .map(|member| quote! {#ident::#member(token) => token.span,})
                .collect::<Vec<_>>();
            let node_arms = items
                .iter()
                .map(|member| quote! {Token::#member(token) => {
                    let tmp = Ok(#ident::#member(token.to_owned()));
                    parser.advance()?;
                    tmp
                },})
                .collect::<Vec<_>>();
            let seq_members = items
                .iter()
                .map(|member| quote! {#ident::#member(token) => token.seq(other),})
                .collect::<Vec<_>>();
            quote! {
                #[derive(Debug, Clone, PartialEq,Eq)]
                pub enum #ident<'a> {
                    #(#members)*
                }
                impl<'a> #ident<'a> {

                    // Spanless Equal
                    fn seq(&self, other: &#ident) -> bool {
                        match (self, other) {
                            #(#seq_members)*
                            _ => false
                        }
                    }
                }
                impl<'a> Node<'a> for #ident<'a> {
                    fn parse(parser: &mut crate::parser::parse_stream::ParseStream<'a>) -> Result<Self, crate::error::ParseError<'a>>
                    where
                        Self: Sized,
                    {
                        let peek = parser.peek()?;
                        match peek {
                            #(#node_arms)*
                            peek => {
                                Err(
                                    ParseError::ExpectedNode(
                                        ExpectedNode {
                                            got: peek.lexeme(),
                                            expected: stringify!(#ident)
                                        }
                                    )
                                )
                            }
                        }
                    }
                }
                impl<'a> #ident<'a> {
                    pub fn lexeme(&self) -> &'a str {
                        match self {
                            #(#lexeme_arms)*
                            _ => todo!("187")
                        }
                    }
                }
                impl<'a> Spanned for #ident<'a> {
                     fn span(&self) -> Span {
                        match self {
                            #(#spanned_arms)*
                            _ => todo!("195")
                        }
                    }
                }

            }
        })
        .collect::<TokenStream2>();
    let lexeme_arms = punctuation
        .0
        .iter()
        .map(|item| {
            let ident = &item.1;
            quote! {
                Token::#ident(token) => token.lexeme,
            }
        })
        .collect::<Vec<_>>();
    let lexemes: Vec<TokenStream2> = lexemes
        .iter()
        .map(|(char, arm)| {
            let span = arm.1.span();
            let lit = syn::LitChar::new(*char, span);
            let arm = &arm.0;

            fn body(arm: &LexArm, span: &Span) -> TokenStream2 {
                let ident = &arm.ident;
                let mut nested: Vec<TokenStream2> = vec![];
                for (char, arm) in arm.children.iter() {
                    let lit = syn::LitChar::new(*char, *span);
                    let a = body(arm, span);
                    nested.push(quote! {
                        #lit => {#a},
                    })
                }
                let nested = if !nested.is_empty() {
                    quote! {
                        if let Some(peek) = self.peek() {
                            match peek {
                                #(#nested)*
                                _ => {}
                            }
                        }
                    }
                } else {
                    quote! {}
                };
                quote! {
                    self.advance().unwrap();
                    #nested
                    let tmp = #ident {
                        span: self.new_span(start),
                        lexeme: &self.src[start..self.src_pos]
                    };
                    return Token::#ident(tmp);
                }
            }

            let tlitbody = body(arm, &span);
            quote! {
                #lit => {
                    let start = self.src_pos;
                    #tlitbody
                },
            }
        })
        .collect();
    let spanned = punctuation
        .0
        .iter()
        .map(|item| {
            let ident = &item.1;
            quote! {
                Token::#ident(token) => token.span(),
            }
        })
        .collect::<Vec<_>>();
    let spanless_equals = punctuation
        .0
        .iter()
        .map(|item| {
            let ident = &item.1;
            quote! {
                (Token::#ident(token), Token::#ident(other)) => &token.seq(&other),
            }
        })
        .collect::<Vec<_>>();
    quote! {
        #(#struct_defs)*
        #groups
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub enum Token<'a> {
            #[default]
            None,
            #(#token_items)*
        }
        impl<'a> Token<'a> {
            // Spanless Equal
            pub fn seq(&self, other: &Token) -> bool {
                match (self, other) {
                    #(#spanless_equals)*
                    _ => false
                }
            }
        }
        impl<'a> Token<'a> {
            pub fn lexeme(&self) -> &'a str {
                match &self {
                    #(#lexeme_arms)*
                    x => "None"
                }
            }
        }
        impl<'a> Spanned for Token<'a> {
            fn span(&self) -> Span {
                match &self {
                    #(#spanned)*
                    x => todo!("245")
                }
            }
        }
        impl<'a> Lexer <'a> {
            pub fn lex_syntax(&mut self) -> Token<'a> {
                let char = self.peek().unwrap();
                match char {
                    #(#lexemes)*
                    _ => todo!("286"),
                }
            }
        }

    }
    .into()
}
