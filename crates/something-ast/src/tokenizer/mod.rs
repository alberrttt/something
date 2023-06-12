use crate::prelude::*;
use std::{
    error::Error,
    fmt::Display,
    ops::{Deref, DerefMut, Index},
};
#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    starting: usize,
    current: usize,
    line: usize,
    // this is relative to the start of the line
    line_current: usize,
}
pub mod delimiter;
use super::error;
pub mod ident;
pub mod list;
pub mod lit;
pub mod program_file;
pub mod to_string;
pub mod token;
pub mod traits;
use error::ParseError;
use ident::*;
use lit::*;
use token::*;
pub use traits::{Parse, ParsingDisplay};

use crate::create_token;
use crate::span;
pub mod prelude {
    pub use super::super::prelude::*;
    pub use super::Tokens;
    pub use super::{
        super::error::{self, *},
        delimiter::{self, *},
        ident::{self, *},
        list::{self, *},
        lit::{self, *},
        to_string::{self, *},
        traits::{self, *},
    };
    #[macro_use]
    pub use super::token::{*, self};
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tokens(pub Vec<Token>, pub usize);

impl Deref for Tokens {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Tokens {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Index<usize> for Tokens {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl From<Vec<Token>> for Tokens {
    fn from(tokens: Vec<Token>) -> Self {
        Self(tokens, 0)
    }
}
impl IntoIterator for Tokens {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl From<&str> for Tokens {
    fn from(tokens: &str) -> Self {
        let mut tokenizer = Tokenizer::new(tokens);
        tokenizer.tokens().unwrap()
    }
}
impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for token in &self.0 {
            write!(f, "{:?}, ", token)?;
        }
        write!(f, "]")?;
        std::result::Result::Ok(())
    }
}

impl Tokens {
    pub fn new() -> Self {
        Self(Vec::new(), 0)
    }
    pub fn parse<T>(&mut self) -> ParseResult<T>
    where
        T: Parse,
        T: Clone + std::fmt::Debug + Clone,
    {
        T::parse(self)
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
    pub fn advance(&mut self) -> Option<&Token> {
        self.1 += 1;
        self.0.get(self.1 - 1)
    }
    pub fn peek(&self) -> Option<&Token> {
        self.0.get(self.1)
    }

    pub fn peek_n(&self, n: isize) -> Option<&Token> {
        let i: usize = ((self.1 as isize) + n).try_into().unwrap();
        self.0.get(i)
    }
    pub fn peek1(&self) -> Option<&Token> {
        self.0.get(self.1 + 1)
    }
    pub fn peek2(&self) -> Option<&Token> {
        self.0.get(self.1 + 2)
    }
    pub fn peek3(&self) -> Option<&Token> {
        self.0.get(self.1 + 3)
    }

    pub fn step<R>(&mut self, F: impl FnOnce(&mut Self) -> ParseResult<R>) -> ParseResult<R> {
        let starting = self.1;
        let stepped = F(self);
        match stepped {
            Ok(ok) => Ok(ok),
            Err(e) => {
                self.1 = starting;
                Err(e)
            }
            Recoverable => {
                self.1 = starting;
                Recoverable
            }
        }
    }
}
impl Tokenizer<'_> {
    pub fn tokens(&mut self) -> ParseResult<Tokens> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if let Ok(token) = token {
                match token {
                    Token::Eof(_) => break,
                    Token::Whitespace(_) => {}
                    _ => tokens.push(token),
                }
            } else if let Err(e) = token {
                return Err(e);
            }
        }
        Ok(Tokens(tokens, 0))
    }
}
impl<'a> Tokenizer<'a> {
    fn identifier(&mut self) -> ParseResult<Ident> {
        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => {
                    self.advance();
                }
                _ => break,
            }
        }
        let lexeme = self.input[self.starting..self.current].to_string();

        Ok(Ident {
            name: lexeme,
            span: span![self.starting, self.current, self.line, self.line_current],
        })
    }
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            starting: 0,
            current: 0,
            line: 1,
            line_current: 0,
        }
    }

    fn next_token(&mut self) -> ParseResult<Token> {
        if self.current >= self.input.len() {
            return Ok(create_token!(self, Eof));
        }
        self.starting = self.current;

        let c = self.advance().unwrap();
        match c {
            'a'..='z' | 'A'..='Z' => {
                let ident = self.identifier()?;
                match ident.name.as_ref() {
                    "false" => return Ok(Token::Lit(Literal::new_bool(ident.span, true))),
                    "true" => return Ok(Token::Lit(Literal::new_bool(ident.span, true))),
                    _ => {}
                }
                let tmp: Token =
                    something_dev_tools::tokens_ident!(If, Let, While, Return, For, Fn);
                Ok(tmp)
            }
            '0'..='9' => Ok(Token::Lit(self.number()?)),
            '"' => Ok(Token::Lit(self.string()?)),
            '=' => Ok(if self.try_consume('=').is_ok() {
                create_token!(self, EqualEqual)
            } else {
                create_token!(self, Equal)
            }),
            '>' => Ok(if self.try_consume('=').is_ok() {
                create_token!(self, GreaterEqual)
            } else {
                create_token!(self, Greater)
            }),
            '<' => Ok(if self.try_consume('=').is_ok() {
                create_token!(self, LessEqual)
            } else if self.try_consume('-').is_ok() {
                create_token!(self, LeftArrow)
            } else {
                create_token!(self, Less)
            }),
            ';' => Ok(create_token!(self, Semicolon)),
            '(' => Ok(Token::Parentheses(self.paren_delimiter())),
            '[' => Ok(Token::Brackets(self.bracket_delimiter())),
            '{' => Ok(Token::Braces(self.brace_delimiter())),
            ')' => Ok(Token::ClosingParen(SpanShell {
                span: span![self.starting, self.current, self.line, self.line_current],
            })),
            ']' => Ok(Token::ClosingBracket(SpanShell {
                span: span![self.starting, self.current, self.line, self.line_current],
            })),
            '}' => Ok(Token::ClosingBrace(SpanShell {
                span: span![self.starting, self.current, self.line, self.line_current],
            })),
            '$' => Ok(create_token!(self, Dollar)),

            // '(' => Ok(create_token!(self, LeftParen)),
            // ')' => Ok(create_token!(self, RightParen)),
            // '{' => Ok(create_token!(self, LeftBrace)),
            // '}' => Ok(create_token!(self, RightBrace)),
            // '[' => Ok(create_token!(self, LeftBracket)),
            // ']' => Ok(create_token!(self, RightBracket)),
            ',' => Ok(create_token!(self, Comma)),
            '#' => Ok(create_token!(self, Hash)),
            ':' => Ok(create_token!(self, Colon)),
            '+' => {
                if self.try_consume('=').is_ok() {
                    Ok(create_token!(self, PlusEqual))
                } else {
                    Ok(create_token!(self, Plus))
                }
            }
            '-' => {
                if self.try_consume('>').is_ok() {
                    Ok(create_token!(self, RightArrow))
                } else if self.try_consume('=').is_ok() {
                    Ok(create_token!(self, MinusEqual))
                } else {
                    Ok(create_token!(self, Minus))
                }
            }
            '*' => {
                if self.try_consume('=').is_ok() {
                    Ok(create_token!(self, StarEqual))
                } else {
                    Ok(create_token!(self, Star))
                }
            }
            '/' => {
                if self.try_consume('=').is_ok() {
                    Ok(create_token!(self, SlashEqual))
                } else {
                    Ok(create_token!(self, Slash))
                }
            }
            '\n' => {
                self.line += 1;
                self.line_current = 0;
                Ok(create_token!(self, Whitespace))
            }
            x if x.is_whitespace() => Ok(create_token!(self, Whitespace)),
            x => Err(ParseError::Generic(format!("Error with `{}`", x.to_string())).into()),
        }
    }
    /// if it matches, it will consume, if not it will return Err
    fn try_consume(&mut self, expected: char) -> ParseResult<char> {
        if self.peek() == Some(expected) {
            let got = self.advance().unwrap();
            Ok(got)
        } else {
            Err(ParseError::Generic(
                format!("Expected {}, got {:?}", expected, self.peek()).into(),
            ))
        }
    }
    fn string(&mut self) -> ParseResult<Literal> {
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                break;
            }
            self.advance();
        }
        let span = span![self.starting, self.current, self.line, self.line_current];
        let lexeme = self.input[self.starting + 1..self.current - 1].to_owned();
        Ok(Literal::new_str(span, lexeme))
    }
    fn number(&mut self) -> ParseResult<Literal> {
        while let Some(c) = self.peek() {
            if c.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }
        let span = span![self.starting, self.current, self.line, self.line_current];
        let lexeme = match self.input[self.starting..self.current].parse::<f64>() {
            std::result::Result::Ok(ok) => ok,
            std::result::Result::Err(err) => return Err(ParseError::Boxed(Box::new(err))),
        };

        Ok(Literal::new_num(span, lexeme))
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.current)
    }
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.line_current += 1;
        self.input.chars().nth(self.current - 1)
    }
}
