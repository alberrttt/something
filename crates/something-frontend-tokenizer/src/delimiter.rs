use crate::tokens::SpanShell;

use crate::{tokens::Span, Token, Tokenizer};
#[derive(Debug, Clone, PartialEq, Eq)]
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
                Token::ClosingBrace(SpanShell { span: _tmp }) => todo!(),
                Token::ClosingBracket(SpanShell { span: _tmp }) => todo!(),
                Token::ClosingParen(SpanShell { span: tmp }) => {
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
                Token::ClosingBrace(SpanShell { span: _tmp }) => todo!(),
                Token::ClosingBracket(SpanShell { span: tmp }) => {
                    span.end = tmp.end;
                    break;
                }
                Token::ClosingParen(SpanShell { span: _tmp }) => todo!(),
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
                Token::ClosingBrace(SpanShell { span: tmp }) => {
                    span.end = tmp.end;
                    break;
                }
                Token::ClosingBracket(SpanShell { span: _tmp }) => todo!(),

                Token::ClosingParen(SpanShell { span: _tmp }) => todo!(),
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
