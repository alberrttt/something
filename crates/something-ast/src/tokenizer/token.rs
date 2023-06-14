use super::prelude::*;
use crate::prelude::*;
use casey::lower;
use std::convert::Infallible;
use std::{error::Error, fmt::Formatter};
macro_rules! define_token {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Default)]
        pub struct $name {
            pub span: Span,
        }
        impl AppendTokens for $name {
            fn append_tokens(&self, tokens: &mut Tokens)
            where
                Self: Sized,
            {
                let tmp = Token::$name(self.clone());
                tokens.push(tmp);
            }
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
        impl Parse for $name {
            fn parse(input: &mut Tokens) -> ParseResult<Self> {
                let token = input.advance()?;
                if let Token::$name(token) = token {
                    Ok(token.clone())
                } else {
                    Err((ParseError::ExpectedToken(Token::$name(Self::default()), token.clone())))
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
            Lit(Literal),
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
        impl ParsingDisplay for Token {
            fn display(&self) -> String
            where
                Self: Sized,
            {

                match self {
                    Token::Ident(i) => i.display(),
                    Token::Lit(l) => l.display(),
                    $(Token::$keyword(k) => k.display()),+,
                    $(Token::$token(t) => t.display()),+,
                    $(Token::$misc(m) => ParsingDisplay::display(m)),+,
                    Token::Parentheses(_) => "()".to_string(),
                    Token::Braces(_) => "{}".to_string(),
                    Token::Brackets(_) => "[]".to_string(),
                    Token::ClosingParen {..} => ")".to_string(),
                    Token::ClosingBrace {..} => "}".to_string(),
                    Token::ClosingBracket {..} => "]".to_string(),
                }


            }

            fn placeholder() -> String
            where
                Self: Sized,
            {
                stringify!($name).into()
            }
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
            define_token!($keyword);
            impl std::fmt::Display for $keyword {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", lower!(stringify!($keyword)))
                }
            }
        )+
        $(
            define_token!($token);
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
            impl ParsingDisplay for $misc {
                fn display(&self) -> String
                where
                    Self: Sized,
                {
                    lower!(stringify!($misc)).into()
                }

                fn placeholder() -> String
                where
                    Self: Sized,
                {
                    stringify!($misc).into()
                }
            }
        )+

    };
}

#[macro_export]
macro_rules! create_token {
    [$self:ident, $name:ident] => {
        Token::$name($name {
            span: span![$self.starting, $self.current, $self.line, $self.line_current],
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

    pub line: usize,
    pub line_start: usize,
}
impl Span {
    pub fn new(start: usize, end: usize, line: usize, line_start: usize) -> Self {
        Self {
            start,
            end,
            line,
            line_start,
        }
    }
}
#[macro_export]
macro_rules! span {
    [$x:expr, $y:expr, $line: expr, $line_start: expr] => {
        Span {
            start: $x,
            end: $y,
            line: $line,
            line_start: $line_start,
        }
    };
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
