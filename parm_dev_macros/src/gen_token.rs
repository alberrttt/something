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
        let ident_name = syn::LitStr::new(&ident.to_string().to_lowercase(), ident.span());

        let mut get_lexemes = |attribute: &Attribute| {
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
            if !path.is_ident("lexeme") {
                return None;
            }
            let Meta::NameValue(name_value) = &attribute.meta else {
                return None;
            };
            let Expr::Lit(lit) = &name_value.value else {
                return None;
            };
            let Lit::Str(str) = &lit.lit else {
                return None;
            };
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
                let arm = prev_arm.children.entry(char).or_insert(LexArm {
                    char,
                    children: HashMap::new(),
                    ident: ident.clone(),
                });
                prev_arm = arm;
            }
            Some(lexeme)
        };
        let mut lexeme = ident_name.clone();
        let mut create_impl = true;
        match &item.0 {
            Some(attributes) => {
                for attribute in attributes {
                    if let Some(lexeme_) = get_lexemes(attribute) {
                        lexeme = LitStr::new(lexeme_.as_str(), ident.span());
                    }
                    if attribute.path().is_ident("no_impl") {
                        create_impl = false;
                    }
                }
            }
            None => {}
        };
        if !create_impl {
            continue;
        }
        let tmp = quote! {
            #[derive(Clone,Copy,PartialEq,Eq,Hash)]
            pub struct #ident<'a> {
                pub lexeme: &'a str,
                pub span: Span
            }
            impl<'a> TreeDisplay for #ident<'a> {
                fn tree(&self) -> Tree {
                    Tree::new(stringify!(#ident))
                        .lexeme(self.lexeme)
                }
            }
            impl Default for #ident<'_> {
                fn default() -> Self {
                    Self {
                        lexeme: #lexeme,
                        span: Span::default()
                    }
                }
            }
            impl<'a> std::fmt::Debug for #ident<'a> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} {}",self.lexeme, stringify!(#ident))
                }
            }

            impl<'a> #ident<'a> {
                // Spanless Equal
                fn spanless_eq(&self, other: &#ident) -> bool {
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
                    parse_stream: &mut crate::ast::parser::parse_stream::ParseStream<'a>,
                ) -> ParseResult<'a, Self>
                where
                    Self: Sized,
                {
                    let peeked = match parse_stream.peek() {
                        Ok(peeked) => peeked,
                        Err(err) => return ParseError::err(
                            ErrorKind::EndOfTokens(EndOfTokens {
                                expected: Some(#ident_name)
                            }),
                            parse_stream.tokens,
                            parse_stream.src_file,
                        ),
                    };
                    if let Token::#ident(peeked) = peeked {
                        let tmp = Ok(peeked.clone());
                        parse_stream.advance()?;
                        tmp
                    } else {
                            ParseError::err(
                                ErrorKind::ExpectedToken(
                                    ExpectedToken {
                                        got: peeked.to_owned(),
                                        expected: Token::#ident(Self::default()),
                                        location: parse_stream.current
                                    }
                                ),
                                parse_stream.tokens,
                                parse_stream.src_file,
                            )
                    }
                }
            }
            impl <'a> From<#ident<'a>> for Token<'a> {
                fn from(value: #ident<'a>) -> Token<'a> {
                    Token::#ident(value)
                }
            }
        };
        struct_defs.push(tmp);
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
        mut token_items,
        struct_defs,
        groups: map_groups,
        lexemes,
    } = generate_struct_defs_token_items(&punctuation);

    let groups = map_groups
        .iter()
        .map(|(group_name, items)| {
            let group_name = format_ident!("{}", group_name);
            
            let members = items
                .iter()
                .map(|ident| quote! {#ident(#ident<'a>),})
                .collect::<Vec<_>>();
            let lexeme_arms = items
                .iter()
                .map(|member| quote! {#group_name::#member(token) => token.lexeme,})
                .collect::<Vec<_>>();
            let spanned_arms = items
                .iter()
                .map(|member| quote! {#group_name::#member(token) => token.span,})
                .collect::<Vec<_>>();
            let node_arms = items
                .iter()
                .map(|member| quote! {Token::#member(token) => {
                    let tmp = Ok(#group_name::#member(token.to_owned()));
                    parse_stream.advance()?;
                    tmp
                },})
                .collect::<Vec<_>>();
            let spanless_eq_members = items
                .iter()
                .map(|member| quote! {(#group_name::#member(token),#group_name::#member(other)) => token.spanless_eq(&other),})
                .collect::<Vec<_>>();
            let is_member_arms: Vec<TokenStream2> = items
                .iter()
                .map(|member| quote! {Token::#member(_) => true,})
                .collect::<Vec<_>>();
            let from_token_arms = items
                .iter().map(|member| quote! {
                    Token::#member(token) => #group_name::#member(token),
                }).collect::<Vec<_>>();
                token_items.push(quote! {
                    #group_name(#group_name<'a>),
                });
            let tree_display_arms = items
                .iter()
                .map(|member| quote! {#group_name::#member(token) => token.tree(),})
                .collect::<Vec<_>>();
            quote! {
                #[derive(Debug, Clone, PartialEq,Eq,Default)]
                pub enum #group_name<'a> {
                    #[default]
                    None,
                    #(#members)*
                   
                }
                impl<'a> TreeDisplay for #group_name<'a> {
                    fn tree(&self) -> Tree {
                        match self {
                            #(#tree_display_arms)*
                            _ => todo!()
                        }
                    }
                }
                impl<'a> From<Token<'a>> for #group_name<'a> {
                    fn from(token: Token<'a>) -> #group_name<'a> {
                        match token {
                            #(#from_token_arms)*
                            _ => todo!()
                        }
                    }
                }
                impl<'a> #group_name<'a> {

                    // Spanless Equal
                    pub fn spanless_eq(&self, other: &#group_name) -> bool {
                        match (self, other) {
                            #(#spanless_eq_members)*
                            _ => false
                        }
                    }

                    pub fn token_is_member(token: &Token<'a>) -> bool {
                        match token {
                            #(#is_member_arms)*
                            _ => false
                        }
                    }
                }

                impl<'a> Node<'a> for #group_name<'a> {
                    fn parse(parse_stream: &mut crate::ast::parser::parse_stream::ParseStream<'a>) -> ParseResult<'a, Self>
                    where
                        Self: Sized,
                    {
                        let peek = parse_stream.peek()?;
                       let location  = parse_stream.current_location_in_file();

                        match peek {
                            #(#node_arms)*
                            peek => {
                                    ParseError::err(
                                        ErrorKind::ExpectedToken(
                                            ExpectedToken {
                                                got: peek.clone(),
                                                expected: Token::#group_name(Self::default()),
                                                location,
                                            }
                                        ),
                                        parse_stream.tokens,
                                        parse_stream.src_file,
                                    )
                            }
                        }
                    }
                }
                impl<'a> #group_name<'a> {
                    pub fn lexeme(&self) -> &'a str {
                        match self {
                            #(#lexeme_arms)*
                            _ => todo!("187")
                        }
                    }
                }
                impl<'a> Spanned for #group_name<'a> {
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
    let mut spanned = punctuation
        .0
        .iter()
        .map(|item| {
            let ident = &item.1;
            quote! {
                Token::#ident(token) => token.span(),
            }
        })
        .collect::<Vec<_>>();
    for key in map_groups.keys() {
        let key = format_ident!("{}", key);
        spanned.push(quote! {
            Token::#key(token) => token.span(),
        })
    }
    let spanless_equals = punctuation
        .0
        .iter()
        .map(|item| {
            let ident = &item.1;
            quote! {
                (Token::#ident(token), Token::#ident(other)) => token.spanless_eq(&other),
            }
        })
        .collect::<Vec<_>>();
    let create_display_node_arms = punctuation
        .0
        .iter()
        .map(|item| {
            let ident = &item.1;
            quote! {
                Token::#ident(token) => token.create_display_node(),
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
            pub fn spanless_eq(&self, other: &Token) -> bool {
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
                    x => todo!("{:?}", x)
                }
            }
        }
        impl<'a> Lexer <'a> {
            pub fn lex_syntax(&mut self) -> Token<'a> {
                let char = self.peek().unwrap();
                match char {
                    #(#lexemes)*
                   char => todo!("{char}"),
                }
            }
        }

    }
    .into()
}
