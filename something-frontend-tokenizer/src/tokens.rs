use std::{error::Error, fmt::Formatter};

use crate::Tokens;

macro_rules! DefineTokens {
    ([$($keyword:ident),+],[$([$t:tt] => $token:ident),+],[$($misc:ident),+]) => {
        #[derive(Debug)]
        pub enum Token{
            Ident(Ident),
            Lit(crate::Literal),
            $($keyword($keyword)),+,
            $($token($token)),+,
            $($misc($misc)),+,

        }
        $(

            pub struct $keyword {
                pub(crate)span: Span
            }

            impl std::fmt::Debug for $keyword {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", stringify!($name))
                }
            }
        )+
        $(

            pub struct $token {
                pub(crate)span: Span
            }

            impl std::fmt::Debug for $token {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", stringify!($token))
                }
            }
        )+
        $(

            pub struct $misc {
                pub(crate)span: Span
            }

            impl std::fmt::Debug for $misc {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", stringify!($misc))
                }
            }
        )+
    };
}
pub trait Parse: Sized {
    fn parse(input: Tokens) -> Result<Self, Box<dyn Error>>;
}
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl std::fmt::Debug for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
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
        [,]  => Comma
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
