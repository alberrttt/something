use crate::{
    tokens::{Parse, Span},
    Token, Tokenizer, Tokens,
};

#[derive(Debug, Clone)]
pub struct Delimiter<const Open: char, const Close: char> {
    pub tokens: Vec<Token>,
    pub span: Span,
}

impl Tokenizer<'_> {
    pub fn paren_delimiter(&mut self) -> Delimiter<'(', ')'> {
        let mut inside: Vec<Token> = Vec::new();
        let mut span: Span = Span {
            start: self.starting,
            end: usize::MAX,
        };
        while let Ok(token) = self.next_token() {
            match token {
                Token::ClosingBrace { span } => todo!(),
                Token::ClosingBracket { span } => todo!(),
                Token::ClosingParen { span: tmp } => {
                    span.end = tmp.end;
                    break;
                }
                Token::Whitespace(_) => {}
                _ => {
                    inside.push(token);
                }
            }
        }

        Delimiter {
            tokens: inside,
            span,
        }
    }
    pub fn bracket_delimiter(&mut self) -> Delimiter<'[', ']'> {
        let mut inside: Vec<Token> = Vec::new();
        let mut span: Span = Span {
            start: self.starting,
            end: usize::MAX,
        };
        while let Ok(token) = self.next_token() {
            match token {
                Token::ClosingBrace { span } => todo!(),
                Token::ClosingBracket { span: tmp } => {
                    span.end = tmp.end;
                    break;
                }
                Token::ClosingParen { span: tmp } => todo!(),
                Token::Whitespace(_) => {}
                _ => {
                    inside.push(token);
                }
            }
        }

        Delimiter {
            tokens: inside,
            span,
        }
    }
    pub fn brace_delimiter(&mut self) -> Delimiter<'{', '}'> {
        let mut inside: Vec<Token> = Vec::new();
        let mut span: Span = Span {
            start: self.starting,
            end: usize::MAX,
        };
        while let Ok(token) = self.next_token() {
            match token {
                Token::ClosingBrace { span: tmp } => {
                    span.end = tmp.end;
                    break;
                }
                Token::ClosingBracket { span: tmp } => todo!(),

                Token::ClosingParen { span: tmp } => todo!(),
                Token::Whitespace(_) => {}
                _ => {
                    inside.push(token);
                }
            }
        }

        Delimiter {
            tokens: inside,
            span,
        }
    }
}
