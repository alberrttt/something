use crate::traits::ParsingDisplay;
use crate::Parse;
use crate::Tokens;
use casey::lower;

use std::{error::Error, fmt::Formatter};
macro_rules! create_token {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Default)]
        pub struct $name {
            pub span: Span,
        }
        impl ParsingDisplay for $name {
            fn display(&self) -> String
            where
                Self: Sized,
            {
                format!("{}", self)
            }

            fn placeholder() -> String
            where
                Self: Sized,
            {
                stringify!($name).into()
            }
        }
        use crate::error::*;
        impl Parse for $name {
            fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
                let token = input.advance().clone();
                if let Some(Token::$name(token)) = token {
                    Ok(token.clone())
                } else {
                    use crate::error::*;
                    Err((ParseError::ExpectedToken(
                        Token::$name(Self::default()),
                        token.cloned().unwrap_or_else(|| Token::Eof(Eof::default())),
                    )))
                }
            }
        }
    };
}
use super::delimiter::*;
use super::ident::*;
use something_dev_tools::Span;
#[derive(Debug, Clone, PartialEq, Eq, Span)]
pub struct SpanShell {
    pub span: Span,
}
macro_rules! DefineTokens {
    ([$($keyword:ident),+],[$([$t:tt] => $token:ident),+],[$($misc:ident),+]) => {
        #[derive( Clone, PartialEq, Eq, Span)]
        pub enum Token{
            Ident(Ident),
            Lit(crate::Literal),
            $($keyword($keyword)),+,
            $($token($token)),+,
            $($misc($misc)),+,
            Parentheses(Delimiter<'(',')'>),
            Braces(Delimiter<'{','}'>),
            Brackets(Delimiter<'[',']'>),
            ClosingParen(SpanShell),
            ClosingBrace(SpanShell),
            ClosingBracket(SpanShell),

        }
        impl std::fmt::Display for Token {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Token::Ident(i) => write!(f, "{}", i),
                    Token::Lit(l) => write!(f, "{}", l),
                    $(Token::$keyword(k) => write!(f, "{}", k),)+
                    $(Token::$token(t) => write!(f, "{}", t),)+
                    $(Token::$misc(m) => write!(f, "{}", m),)+
                    Token::Parentheses(_) => write!(f, "()"),
                    Token::Braces(_) => write!(f, "{{}}"),
                    Token::Brackets(_) => write!(f, "[]"),
                    Token::ClosingParen {..} => write!(f, ")"),
                    Token::ClosingBrace {..} => write!(f, "}}"),
                    Token::ClosingBracket {..} => write!(f, "]"),
                }
            }
        }
        impl std::fmt::Debug for Token {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Token::Ident(i) => write!(f, "{:?}", i),
                    Token::Lit(l) => write!(f, "{:?}", l),
                    $(Token::$keyword(k) => write!(f, "{:?}", k),)+
                    $(Token::$token(t) => write!(f, "{:?}", t),)+
                    $(Token::$misc(m) => write!(f, "{:?}", m),)+
                    Token::Parentheses(_) => write!(f, "()"),
                    Token::Braces(_) => write!(f, "{{}}"),
                    Token::Brackets(_) => write!(f, "[]"),
                    Token::ClosingParen {..} => write!(f, ")"),
                    Token::ClosingBrace {..} => write!(f, "}}"),
                    Token::ClosingBracket {..} => write!(f, "]"),
                }
            }
        }

        $(
            create_token!($keyword);
            impl std::fmt::Display for $keyword {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", lower!(stringify!($keyword)))
                }
            }
        )+
        $(
            create_token!($token);
            impl std::fmt::Display for $token {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", stringify!($t))
                }
            }
        )+
        $(
            #[derive(Clone, PartialEq, Eq, Default)]
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
                    write!(f, "{}", lower!(stringify!($misc)))
                }
            }
        )+

    };
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
    [If, Fn, Let, Return, While, For],
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
        [#] => Hash,
        [->] => RightArrow,
        [<-] => LeftArrow,
        [$] => Dollar,
        [+=] => PlusEqual,
        [/=] => SlashEqual,
        [-=] => MinusEqual,
        [*=] => StarEqual
    ], [
        Eof,
        Whitespace
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Span {
    pub start: usize,

    pub end: usize,
}
impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
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

impl ParsingDisplay for () {
    fn display(&self) -> String
    where
        Self: Sized,
    {
        "".to_string()
    }

    fn placeholder() -> std::string::String
    where
        Self: Sized,
    {
        "<empty>".into()
    }
}
