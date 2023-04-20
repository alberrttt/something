use std::error::Error;

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    lexeme: &'a str,
    span: Span,
}
#[derive(Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

pub struct Tokenizer<'a> {
    input: &'a str,
    starting: usize,
    current: usize,
}
macro_rules! Token {
    ($self: ident, $kind:ident) => {
        Ok(Token {
            kind: TokenKind::$kind,
            lexeme: &$self.input[$self.starting..$self.current],
            span: Span {
                start: $self.starting,
                end: $self.current,
            },
        })
    };
}
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Let,
    Identifier,
    If,
    Fn,
    While,
    For,
    Return,
    False,
    True,

    Number,
    String,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Equal,
    EqualEqual,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,
    Semicolon,
    Whitespace,
    Eof,
}
impl<'a> Tokenizer<'a> {
    fn identifier(&mut self) -> Result<Token<'a>, Box<dyn Error>> {
        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    self.advance();
                }
                _ => break,
            }
        }
        let lexeme = &self.input[self.starting..self.current];
        use something_dev_tools::TOKENS;

        TOKENS!(If, Fn, Let, False, True, Return, While, For);
        Ok(Token {
            kind: TokenKind::Identifier,
            lexeme,
            span: Span {
                start: self.starting,
                end: self.current,
            },
        })
    }
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            starting: 0,
            current: 0,
        }
    }
    pub fn all_tokens(&mut self) -> Result<Vec<Token<'a>>, Box<dyn Error>> {
        let mut tokens = Vec::new();
        while let Ok(token) = self.next_token() {
            if token.kind == TokenKind::Eof {
                tokens.push(token);
                break;
            }
            if token.kind == TokenKind::Whitespace {
                continue;
            }
            tokens.push(token)
        }
        Ok(tokens)
    }
    pub fn next_token(&mut self) -> Result<Token<'a>, Box<dyn Error>> {
        if self.current >= self.input.len() {
            return Token!(self, Eof);
        }
        self.starting = self.current;
        let c = self.advance().unwrap();
        match c {
            'a'..='z' | 'A'..='Z' => self.identifier(),
            '0'..='9' => self.number(),
            '"' => self.string(),
            '=' => {
                if self.try_consume('=').is_ok() {
                    Token!(self, EqualEqual)
                } else {
                    Token!(self, Equal)
                }
            }
            '>' => {
                if self.try_consume('=').is_ok() {
                    Token!(self, GreaterEqual)
                } else {
                    Token!(self, Greater)
                }
            }
            '<' => {
                if self.try_consume('=').is_ok() {
                    Token!(self, LessEqual)
                } else {
                    Token!(self, Less)
                }
            }
            ';' => Token!(self, Semicolon),
            '(' => Token!(self, LeftParen),
            ')' => Token!(self, RightParen),
            '{' => Token!(self, LeftBrace),
            '}' => Token!(self, RightBrace),
            '[' => Token!(self, LeftBracket),
            ']' => Token!(self, RightBracket),
            ',' => Token!(self, Comma),
            x if x.is_whitespace() => Token!(self, Whitespace),
            x => Err(x.to_string().into()),
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
    fn string(&mut self) -> Result<Token<'a>, Box<dyn Error>> {
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                break;
            }
            self.advance();
        }
        Ok(Token {
            kind: TokenKind::String,
            lexeme: &self.input[self.starting..self.current],
            span: Span {
                start: self.starting,
                end: self.current,
            },
        })
    }
    fn number(&mut self) -> Result<Token<'a>, Box<dyn Error>> {
        while let Some(c) = self.peek() {
            if c.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }
        Ok(Token {
            kind: TokenKind::Number,
            lexeme: &self.input[self.starting..self.current],
            span: Span {
                start: self.starting,
                end: self.current,
            },
        })
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.current)
    }
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.input.chars().nth(self.current - 1)
    }
}

pub fn tokenize(input: &str) {
    let mut tokenizer = Tokenizer::new("let a = 1;");
    while let Ok(token) = tokenizer.next_token() {
        println!("{:?}", token);
    }
}
