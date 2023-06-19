use super::{prelude::*, Tokenizer};
use crate::prelude::*;
use casey::lower;

use std::backtrace::Backtrace;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut, Index};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStream(pub Vec<Token>, pub usize);

impl Deref for TokenStream {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TokenStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Index<usize> for TokenStream {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl From<Vec<Token>> for TokenStream {
    fn from(tokens: Vec<Token>) -> Self {
        Self(tokens, 0)
    }
}
impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl From<&str> for TokenStream {
    fn from(tokens: &str) -> Self {
        let mut tokenizer = Tokenizer::new(tokens);
        tokenizer.tokens().unwrap()
    }
}
impl Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for token in &self.0 {
            write!(f, "{:?}, ", token)?;
        }
        write!(f, "]")?;
        std::result::Result::Ok(())
    }
}

impl TokenStream {
    pub fn try_advance(&mut self, target: Token) -> ParseResult<Token> {
        match self.peek() {
            Ok(token) => {
                if *token == target {
                    let token = token.clone();
                    self.advance();
                    return Ok(token);
                } else {
                    return Err(ParseError::ExpectedToken(target, token.clone()));
                }
            }
            Recoverable | Err(_) => return Recoverable, // idk if recoverable is the right thing to do here
                                                        // but, if the self.peek() errors, it usually means that we're at the end of the tokens
        }
        todo!()
    }
    pub fn new() -> Self {
        Self(Vec::new(), 0)
    }

    pub fn previous(&self) -> Option<&Token> {
        self.0.get(self.1 - 1)
    }
    pub fn previous1(&self) -> Option<&Token> {
        self.0.get(self.1 - 2)
    }
    pub fn previous2(&self) -> Option<&Token> {
        self.0.get(self.1 - 3)
    }
    pub fn previous3(&self) -> Option<&Token> {
        self.0.get(self.1 - 4)
    }
    pub fn at_end(&self) -> bool {
        self.1 >= self.0.len()
    }
    pub fn distance_from_end(&self) -> usize {
        self.0.len() - self.1
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn get(&self, i: usize) -> Option<&Token> {
        self.0.get(i)
    }
    pub fn get_mut(&mut self, i: usize) -> Option<&mut Token> {
        self.0.get_mut(i)
    }
    pub fn first(&self) -> Option<&Token> {
        self.0.first()
    }
    pub fn last(&self) -> Option<&Token> {
        self.0.last()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Token> {
        self.0.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Token> {
        self.0.iter()
    }
    pub fn advance(&mut self) -> ParseResult<&Token> {
        self.1 += 1;
        match self.0.get(self.1 - 1) {
            Some(some) => Ok(some),
            None => Err(ParseError::EndOfTokens(Backtrace::capture())),
        }
    }
    pub fn peek(&self) -> ParseResult<&Token> {
        match self.0.get(self.1) {
            Some(token) => Ok(token),
            None => Err(ParseError::EndOfTokens(Backtrace::capture())),
        }
    }

    pub fn peek_n(&self, n: usize) -> ParseResult<&Token> {
        match self.0.get(self.1 + n) {
            Some(token) => Ok(token),
            None => Err(ParseError::EndOfTokens(Backtrace::capture())),
        }
    }

    pub fn peek1(&self) -> ParseResult<&Token> {
        self.peek_n(1)
    }
    pub fn peek2(&self) -> ParseResult<&Token> {
        self.peek_n(2)
    }
    pub fn peek3(&self) -> ParseResult<&Token> {
        self.peek_n(3)
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! define_token {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Default)]
        pub struct $name {
            pub span: Span,
        }
        impl AppendTokens for $name {
            fn append_tokens(&self, tokens: &mut TokenStream)
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
            fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
                let token = parser.advance()?;
                if let Token::$name(token) = token {
                    Ok(token.clone())
                } else {
                    Err((ParseError::ExpectedToken(Token::$name(Self::default()), token.clone())))
                }
            }
        }
    };
}
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

            LeftParen(LeftParen),
            LeftBrace(LeftBrace),
            LeftBracket(LeftBracket),
            RightParen(RightParen),
            RightBrace(RightBrace),
            RightBracket(RightBracket),

            /// If you want to report a parsing error, i.e converting a string to the number
            /// from the source code, and you encounter an error,
            /// you dont have a token to report the error on, so you use this
            Error(SpanShell),
        }
        impl AppendTokens for Token {
            fn append_tokens(&self, token_stream: &mut TokenStream)

            {
               token_stream.push(self.clone());
            }
        }
        pub mod Macros {
            /// Macro for constructing tokens from their actualy syntatic representation
            macro_rules! Tkn {
                $([$keyword] => {
                    $crate::tokenizer::token::$keyword
                };)+
                $([$t] => {
                    $crate::tokenizer::token::$token
                };)+
                $([$misc] => {
                    $crate::tokenizer::token::$misc
                };)+
            }
            pub(crate) use Tkn;
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
                    Token::LeftParen {..} => "(".to_string(),
                    Token::LeftBrace {..} => "{".to_string(),
                    Token::LeftBracket {..} => "[".to_string(),
                    Token::RightParen {..} => ")".to_string(),
                    Token::RightBrace {..} => "}".to_string(),
                    Token::RightBracket {..} => "]".to_string(),
                    Token::Error(_) => "Error".to_string(),
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
                    Token::LeftParen {..} => write!(f, "("),
                    Token::LeftBrace {..} => write!(f, "{{"),
                    Token::LeftBracket {..} => write!(f, "["),

                    Token::RightParen {..} => write!(f, ")"),
                    Token::RightBrace {..} => write!(f, "}}"),
                    Token::RightBracket {..} => write!(f, "]"),
                    Token::Error(_) => write!(f, "Error"),
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
                    Token::LeftParen {..} => write!(f, "("),
                    Token::LeftBrace {..} => write!(f, "{{"),
                    Token::LeftBracket {..} => write!(f, "["),
                    Token::RightParen {..} => write!(f, ")"),
                    Token::RightBrace {..} => write!(f, "}}"),
                    Token::RightBracket {..} => write!(f, "]"),
                    Token::Error(_) => write!(f, "Error"),
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
        define_token!(LeftParen);
        impl std::fmt::Display for LeftParen {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "(")
            }
        }
        define_token!(LeftBrace);
        impl std::fmt::Display for LeftBrace {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{{")
            }
        }
        define_token!(LeftBracket);
        impl std::fmt::Display for LeftBracket {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "[")
            }
        }
        define_token!(RightParen);
        impl std::fmt::Display for RightParen {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, ")")
            }
        }
        define_token!(RightBrace);
        impl std::fmt::Display for RightBrace {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "}}")
            }
        }
        define_token!(RightBracket);
        impl std::fmt::Display for RightBracket {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "]")
            }
        }
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
    };
    [$self:ident, $name:ident no struct] => {
        Token::$name($name { span: span![$self.starting, $self.current, $self.line, $self.line_current], })
    }
}

DefineTokens!(
    [If, Fn, Let, Return, While, For, Use],
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
impl Token {
    pub fn is_closing_delimiter(&self) -> bool {
        matches!(
            self,
            Token::RightParen { .. } | Token::RightBrace { .. } | Token::RightBracket { .. }
        )
    }
}
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
