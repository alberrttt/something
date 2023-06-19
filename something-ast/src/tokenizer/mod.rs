use crate::prelude::*;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    parser: &'a str,
    starting: usize,
    current: usize,
    line: usize,
    // this is relative to the start of the line
    line_current: usize,
}
use super::error;
pub mod ident;
pub mod list;
pub mod lit;
pub mod program_file;
pub mod to_string;
pub mod token;
pub mod traits;
use crate::create_token;
use crate::span;
use error::ParseError;
use ident::*;
use lit::*;
pub use token::TokenStream;
use token::*;
pub use traits::{Parse, ParsingDisplay};
pub mod prelude {
    pub use super::super::prelude::*;
    pub use super::TokenStream;
    pub use super::{
        super::error::{self, *},
        ident::{self, *},
        list::{self, *},
        lit::{self, *},
        to_string::{self, *},
        traits::{self, *},
    };
    #[macro_use]
    pub use super::token::{*, self};
}
impl Tokenizer<'_> {
    pub fn tokens(&mut self) -> ParseResult<TokenStream> {
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
        Ok(TokenStream(tokens, 0))
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
        let lexeme = self.parser[self.starting..self.current].to_string();

        Ok(Ident {
            name: lexeme,
            span: span![self.starting, self.current, self.line, self.line_current],
        })
    }
    pub fn new(parser: &'a str) -> Self {
        Tokenizer {
            parser,
            starting: 0,
            current: 0,
            line: 1,
            line_current: 0,
        }
    }

    fn next_token(&mut self) -> ParseResult<Token> {
        if self.current >= self.parser.len() {
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
                    something_dev_tools::tokens_ident!(If, Let, While, Return, For, Fn, Use);
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
            '(' => Ok(create_token!(self, LeftParen no struct)),
            ')' => Ok(create_token!(self, RightParen no struct)),
            '{' => Ok(create_token!(self, LeftBrace no struct)),
            '}' => Ok(create_token!(self, RightBrace no struct)),
            '[' => Ok(create_token!(self, LeftBracket no struct)),
            ']' => Ok(create_token!(self, RightBracket no struct)),
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
        let lexeme = self.parser[self.starting + 1..self.current - 1].to_owned();
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
        let lexeme = match self.parser[self.starting..self.current].parse::<f64>() {
            std::result::Result::Ok(ok) => ok,
            std::result::Result::Err(err) => return Err(ParseError::ParseFloatError(err)),
        };

        Ok(Literal::new_num(span, lexeme))
    }

    fn peek(&self) -> Option<char> {
        self.parser.chars().nth(self.current)
    }
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.line_current += 1;
        self.parser.chars().nth(self.current - 1)
    }
}
pub(crate) use crate::tokenizer::token::Macros::*;
