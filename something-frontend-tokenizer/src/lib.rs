use std::{error::Error, fmt::Display};
#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    starting: usize,
    current: usize,
}

pub mod tokens;
use tokens::*;
mod lit;
pub use lit::Literal;
pub struct Tokens(pub(crate) Vec<Token>, pub(crate) usize);

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.0 {
            writeln!(f, "{:?},", token)?;
        }
        Ok(())
    }
}
impl Tokenizer<'_> {
    pub fn tokens(&mut self) -> Result<Tokens, Box<dyn Error>> {
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
    fn identifier(&mut self) -> Result<Ident, Box<dyn Error>> {
        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    self.advance();
                }
                _ => break,
            }
        }
        let lexeme = self.input[self.starting..self.current].to_string();

        Ok(Ident {
            name: lexeme,
            span: span![self.starting, self.current],
        })
    }
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            starting: 0,
            current: 0,
        }
    }

    fn next_token(&mut self) -> Result<Token, Box<dyn Error>> {
        if self.current >= self.input.len() {
            return Ok(Token!(self, Eof));
        }
        self.starting = self.current;
        let c = self.advance().unwrap();
        match c {
            'a'..='z' | 'A'..='Z' => {
                let ident = self.identifier()?;
                use something_dev_tools::tokens;
                let tmp: Token = tokens!(If, Let, False, True, While, Return, For, Fn);
                Ok(tmp)
            }
            '0'..='9' => Ok(Token::Lit(self.number()?)),
            '"' => Ok(Token::Lit(self.string()?)),
            '=' => Ok(if self.try_consume('=').is_ok() {
                Token!(self, EqualEqual)
            } else {
                Token!(self, Equal)
            }),
            '>' => Ok(if self.try_consume('=').is_ok() {
                Token!(self, GreaterEqual)
            } else {
                Token!(self, Greater)
            }),
            '<' => Ok(if self.try_consume('=').is_ok() {
                Token!(self, LessEqual)
            } else {
                Token!(self, Less)
            }),
            ';' => Ok(Token!(self, Semicolon)),
            // '(' => Ok(Token!(self, LeftParen)),
            // ')' => Ok(Token!(self, RightParen)),
            // '{' => Ok(Token!(self, LeftBrace)),
            // '}' => Ok(Token!(self, RightBrace)),
            // '[' => Ok(Token!(self, LeftBracket)),
            // ']' => Ok(Token!(self, RightBracket)),
            ',' => Ok(Token!(self, Comma)),
            x if x.is_whitespace() => Ok(Token!(self, Whitespace)),
            x => Err(format!("Error with `{}`", x.to_string()).into()),
        }
    }
    /// if it matches, it will consume, if not it will return Err
    fn try_consume(&mut self, expected: char) -> Result<char, Box<dyn Error>> {
        if self.peek() == Some(expected) {
            let got = self.advance().unwrap();
            Ok(got)
        } else {
            Err(format!("Expected {}, got {:?}", expected, self.peek()).into())
        }
    }
    fn string(&mut self) -> Result<Literal, Box<dyn Error>> {
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                break;
            }
            self.advance();
        }
        let span = span![self.starting, self.current];
        let lexeme = self.input[self.starting..self.current].to_owned();
        Ok(Literal::new_str(span, lexeme))
    }
    fn number(&mut self) -> Result<Literal, Box<dyn Error>> {
        while let Some(c) = self.peek() {
            if c.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }
        let span = span![self.starting, self.current];
        let lexeme = self.input[self.starting..self.current].parse::<f64>()?;

        Ok(Literal::new_num(span, lexeme))
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.current)
    }
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.input.chars().nth(self.current - 1)
    }
}
