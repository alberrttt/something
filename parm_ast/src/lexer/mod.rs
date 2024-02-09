pub mod token;

use parm_common::{Span, Spanned};
use std::char;
use token::{Token, *};

use self::token::{Float, Integer};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer<'src> {
    pub src: &'src str,
    /// position in the entire file
    pub src_pos: usize,
    /// position on the line
    pub line_pos: usize,
    /// current line
    pub line: usize,

    pub tokens: Vec<Token<'src>>,
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
            tokens: Vec::new(),
        }
    }
}
impl<'src> Lexer<'src> {
    pub fn get_token_idx_from_token(&self, token: &Token<'src>) -> Option<usize> {
        self.tokens
            .iter()
            .position(|t| t.span().src_start == token.span().src_start)
    }
    pub fn get_token_idx_from_span(&self, span: &Span) -> Option<usize> {
        self.tokens
            .iter()
            .position(|t| t.span().src_start == span.src_start)
    }
    pub fn get_range_from_span(&self, span: &Span) -> Option<std::ops::Range<usize>> {
        let start = self.get_token_idx_from_span(span)?;
        let end = self
            .tokens
            .iter()
            .position(|t| t.span().src_end == span.src_end)?;
        Some(start..end)
    }
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
        let mut tokens: Vec<Token<'_>> = Vec::new();
        loop {
            if self.src_pos >= self.src.len() {
                break;
            }

            let char = self.peek().unwrap();

            match char {
                '"' => {
                    let start = self.src_pos;
                    self.advance();
                    loop {
                        if self.src_pos >= self.src.len() {
                            break;
                        }
                        let current = self.src.chars().nth(self.src_pos).unwrap();
                        if current == '"' {
                            break;
                        }
                        self.advance();
                    }
                    let lexeme = &self.src[(start + 1)..self.src_pos];
                    let span = Span {
                        src_start: start,
                        src_end: self.src_pos + 1,
                        line_start: self.line_pos - (self.src_pos - start),
                        line: self.line,
                    };
                    tokens.push(Token::StringLiteral(token::StringLiteral { lexeme, span }));
                    self.advance();
                }
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
        self.tokens = tokens.clone();
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

        match lexeme {
            "true" => True { lexeme, span }.into(),
            "false" => False { lexeme, span }.into(),
            "if" => If { lexeme, span }.into(),
            "else" => Else { lexeme, span }.into(),
            "fn" => FnKeyword { lexeme, span }.into(),
            "return" => Return { lexeme, span }.into(),
            "let" => Let { lexeme, span }.into(),
            "mut" => Mut { lexeme, span }.into(),
            "use" => Use { lexeme, span }.into(),
            "loop" => Loop { lexeme, span }.into(),
            "while" => While { lexeme, span }.into(),
            "for" => For { lexeme, span }.into(),
            "in" => In { lexeme, span }.into(),
            "break" => Break { lexeme, span }.into(),
            "continue" => Continue { lexeme, span }.into(),
            "struct" => StructKeyword { lexeme, span }.into(),
            "enum" => Enum { lexeme, span }.into(),
            "impl" => Impl { lexeme, span }.into(),
            "trait" => Trait { lexeme, span }.into(),
            "type" => TypeKeyword { lexeme, span }.into(),
            "pub" => Pub { lexeme, span }.into(),
            "self" => LSelf { lexeme, span }.into(),
            "Self" => USelf { lexeme, span }.into(),
            _ => Identifier { lexeme, span }.into(),
        }
    }
}

#[test]
fn test_lit() {
    let mut lexer = Lexer::from("123 123.456");
    let tokens = lexer.lex();

    let _tmp = tokens.get(0).unwrap();
}
#[test]
fn test_ident() {
    let mut lexer = Lexer::from("let mut false if else ident1 ident2 a_b_c");
    let _tokens = lexer.lex();
}
