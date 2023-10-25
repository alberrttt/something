use std::{cmp::Ordering, fmt::Display, slice, vec};

use parmesan_common::{Span, Spanned};

use crate::lexer::{
    token::{tokens_by_line, Token},
    Lexer,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ErrorMessage<'a> {
    tokens: Vec<Token<'a>>,
    messages: Vec<(Span, String)>,
}

impl<'a> Display for ErrorMessage<'a> {
    /// note: this might be unsafe if the tokens aren't in correct order.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = tokens_by_line(&self.tokens);

        for (i, line) in lines.iter().enumerate() {
            let mut prev_token: *const Token<'a> = std::ptr::null();

            for (i, token) in line.iter().enumerate() {
                if i == 0 {
                    write!(
                        f,
                        "{:whitespace$}",
                        "",
                        whitespace = token.span().line_start
                    )?;
                } else {
                    let prev_token = unsafe { &*prev_token };
                    write!(
                        f,
                        "{:whitespace$}",
                        "",
                        // whitespace = current token start - previous token end
                        whitespace = token.span().line_start
                            - (prev_token.span().line_start + prev_token.span().src_end
                                - prev_token.span().src_start)
                    )?;
                }
                write!(f, "{}", token.lexeme())?;
                prev_token = token
            }
            if i != lines.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
#[test]
fn test_error_message() {
    let mut lexer = Lexer::from("line1 item2\nline2\nline3");
    let tokens = lexer.lex();

    let msg = ErrorMessage {
        tokens,
        messages: vec![],
    };
    assert_eq!(msg.to_string(), "line1 item2\nline2\nline3")
}
pub trait ParseError {
    fn error(&self) -> String;
    fn tokens(&self) -> Vec<Token>;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EndOfTokens<'a> {
    pub tokens: Vec<Token<'a>>,
}
