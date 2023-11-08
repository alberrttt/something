pub mod token;

use parmesan_common::Span;
use std::char;
use token::Token;

use self::token::{Float, Ident, If, Integer};

#[derive(Debug, Clone,PartialEq)]
pub struct Lexer<'src> {
    pub src: &'src str,
    /// position in the entire file
    pub src_pos: usize,
    /// position on the line
    pub line_pos: usize,
    /// current line
    pub line: usize,
}

fn ident_body(char: char) -> bool {
    char.is_ascii_alphanumeric() || char == '_'
}
impl<'src> From<&'src str> for Lexer<'src> {
    fn from(src: &'src str) -> Self {
        Self {
            src,
            src_pos: 0,
            line_pos: 0,
            line: 0,
        }
    }
}
impl<'src> Lexer<'src> {
    /**
     * creates a new span starting at the given position and ends at the current position
     */
    pub fn new_span(&self, start: usize) -> Span {
        Span {
            src_start: start,
            src_end: self.src_pos,
            line_start: self.line_pos - (self.src_pos - start),
            line: self.line,
        }
    }
    pub fn advance(&mut self) -> Option<char> {
        let char = self.src.chars().nth(self.src_pos);
        self.src_pos += 1;
        self.line_pos += 1;
        char
    }
    pub fn advance_n(&mut self, n: usize) -> Option<char> {
        let char = self.src.chars().nth(self.src_pos + n);
        self.src_pos += n;
        self.line_pos += n;
        char
    }
    pub fn peek(&self) -> Option<char> {
        self.src.chars().nth(self.src_pos)
    }
    pub fn peek_next(&self, n: usize) -> Option<char> {
        self.src.chars().nth(self.src_pos + n)
    }

    pub fn lex(&mut self) -> Vec<Token<'src>> {
        let mut tokens = Vec::new();
        loop {
            if self.src_pos >= self.src.len() {
                break;
            }

            let char = self.peek().unwrap();

            match char {
                '_' | 'a'..='z' | 'A'..='Z' => {
                    tokens.push(self.ident());
                }
                '0'..='9' => {
                    tokens.push(self.number());
                }
                '\n' => {
                    self.advance();

                    self.line += 1;
                    self.line_pos = 0;
                }
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                _ => tokens.push(self.lex_syntax()),
            }
        }
        tokens
    }
    pub(crate) fn number(&mut self) -> Token<'src> {
        let start = self.src_pos;
        let mut point = false;
        loop {
            if self.src_pos >= self.src.len() {
                break;
            }
            let current = self.src.chars().nth(self.src_pos).unwrap();
            if current.is_ascii_digit() || (current == '.' && !point) {
                if current == '.' {
                    point = true;
                }
                self.advance();
            } else {
                break;
            }
        }
        if point {
            Float {
                lexeme: &self.src[start..self.src_pos],
                span: self.new_span(start),
            }
            .into()
        } else {
            Integer {
                lexeme: &self.src[start..self.src_pos],
                span: self.new_span(start),
            }
            .into()
        }
    }

    pub(crate) fn ident(&mut self) -> Token<'src> {
        let start = self.src_pos;

        loop {
            if self.src_pos >= self.src.len() {
                break;
            }
            let current = self.src.chars().nth(self.src_pos).unwrap();
            if ident_body(current) {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = &self.src[start..self.src_pos];
        let span = self.new_span(start);
        use parmesan_dev_macros::lower_stringify;
        macro_rules! keyword_match {
            ($($ident:ident),*) => {
                match lexeme {
                    $(
                        lower_stringify!($ident) => $ident {
                            lexeme, span
                        }.into(),
                    )*
                    _ => Ident { lexeme, span }.into(),
                }
            };
        }
        use crate::lexer::token::*;

        keyword_match!(True, False, If, Else, FnKeyword, Return, Let, Mut)
    }
}

#[test]
fn test_lit() {
    let mut lexer = Lexer::from("123 123.456");
    let tokens = lexer.lex();

    let tmp = tokens.get(0).unwrap();
}
#[test]
fn test_ident() {
    let mut lexer = Lexer::from("let mut false if else ident1 ident2 a_b_c");
    let tokens = lexer.lex();
}
