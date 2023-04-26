use std::{error::Error, fmt::Formatter};

use crate::Tokens;
macro_rules! create_token {
    ($name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            pub(crate) span: Span,
        }

        impl Parse for $name {
            fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>> {
                let token = input.advance().clone();
                if let Token::$name(token) = token {
                    Ok(token)
                } else {
                    Err(format!("Expected {}, got {:?}", stringify!($name), token).into())
                }
            }
        }
    };
}
use super::delimiter::*;
use super::ident::*;
macro_rules! DefineTokens {
    ([$($keyword:ident),+],[$([$t:tt] => $token:ident),+],[$($misc:ident),+]) => {
        #[derive(Debug, Clone)]
        pub enum Token{
            Ident(Ident),
            Lit(crate::Literal),
            $($keyword($keyword)),+,
            $($token($token)),+,
            $($misc($misc)),+,
            Paren(Delimiter<'(',')'>),
            Brace(Delimiter<'{','}'>),
            Bracket(Delimiter<'[',']'>),
            ClosingParen {span: Span},
            ClosingBrace {span: Span},
            ClosingBracket {span: Span},

        }
        $(
            create_token!($keyword);
        )+
        $(
            create_token!($token);
        )+
        $(
            #[derive(Clone)]
            pub struct $misc {
                pub(crate)span: Span
            }

            impl std::fmt::Debug for $misc {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", stringify!($misc))
                }
            }

            impl std::fmt::Display for $misc {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", stringify!($misc))
                }
            }
        )+

    };
}
pub trait Parse: Sized {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>>;
}

#[macro_export]
macro_rules! Token {
    [$self:ident, $name:ident] => {
        Token::$name($name {
            span: span![$self.starting, $self.current],
        })
    }

}
DefineTokens!(
    [If, Fn, Let, False, True, Return, While, For],
    [
        [==]  => EqualEqual,
        [=]  => Equal,
        [>=]  => GreaterEqual,
        [>]  => Greater,
        [<=]  => LessEqual,
        [<]  => Less,
        [+]  => Plus,
        [-]  => Minus,
        [*]  => Star,
        [/]  => Slash,
        [!=]  => BangEqual,
        [!]  => Bang,
        [;]  => Semicolon,
        [,]  => Comma,
        [:] => Colon,
        [#] => Hash
    ], [
        Eof,
        Whitespace
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub end: usize,
    pub start: usize,
}
#[macro_export]
macro_rules! span {
    [$x:expr, $y:expr] => {
        Span {
            start: $x,
            end: $y,
        }
    };
}
pub trait Token__: std::fmt::Debug {
    fn span(&self) -> Span;
    fn display(&self) -> String
    where
        Self: Sized;
}
impl Parse for () {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>> {
        Ok(())
    }
}
