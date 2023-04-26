use std::{error::Error, fmt::Display};
#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    starting: usize,
    current: usize,
}

pub mod delimiter;
pub mod ident;
pub mod lit;
pub mod tokens;
use ident::*;
use lit::*;
use tokens::*;
#[derive(Debug, Clone)]
pub struct Tokens(pub(crate) Vec<Token>, pub(crate) usize);
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
impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for token in &self.0 {
            write!(f, "{:?}, ", token)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Tokens {
    pub fn iter(&self) -> std::slice::Iter<'_, Token> {
        self.0.iter()
    }
    pub fn advance(&mut self) -> &Token {
        let token = &self.0[self.1];
        self.1 += 1;
        &token
    }
    pub fn peek(&self) -> Option<&Token> {
        self.0.get(self.1)
    }

    pub fn step<R>(
        &mut self,
        F: impl FnOnce(&mut Self) -> Result<R, Box<dyn Error>>,
    ) -> Result<R, Box<dyn Error>> {
        let starting = self.1;
        let stepped = F(self);
        match stepped {
            Ok(ok) => Ok(ok),
            Err(e) => {
                self.1 = starting;
                Err(e)
            }
        }
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
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => {
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
            '(' => Ok(Token::Paren(self.paren_delimiter().unwrap())),
            '[' => Ok(Token::Bracket(self.bracket_delimiter().unwrap())),
            '{' => Ok(Token::Brace(self.brace_delimiter().unwrap())),
            // '(' => Ok(Token!(self, LeftParen)),
            // ')' => Ok(Token!(self, RightParen)),
            // '{' => Ok(Token!(self, LeftBrace)),
            // '}' => Ok(Token!(self, RightBrace)),
            // '[' => Ok(Token!(self, LeftBracket)),
            // ']' => Ok(Token!(self, RightBracket)),
            ',' => Ok(Token!(self, Comma)),
            '#' => Ok(Token!(self, Hash)),
            ':' => Ok(Token!(self, Colon)),
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
        let lexeme = self.input[self.starting + 1..self.current - 1].to_owned();
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
