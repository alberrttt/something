use crate::{
    tokens::{Parse, Span},
    Token, Tokenizer, Tokens,
};

#[derive(Debug, Clone)]
pub struct Delimiter<const Open: char, const Close: char> {
    pub tokens: Vec<Token>,
}

impl Tokenizer<'_> {
    pub fn paren_delimiter(&mut self) -> Result<Delimiter<'(', ')'>, ()> {
        let mut inside = Vec::new();
        while let Ok(token) = self.next_token() {
            match token {
                Token::ClosingBrace { span } => todo!(),
                Token::ClosingBracket { span } => todo!(),
                Token::ClosingParen { span } => break,
                _ => {
                    inside.push(token);
                }
            }
        }

        Ok(Delimiter { tokens: inside })
    }
    pub fn bracket_delimiter(&mut self) -> Result<Delimiter<'[', ']'>, ()> {
        let mut inside = Vec::new();
        while let Ok(token) = self.next_token() {
            match token {
                Token::ClosingBrace { span } => panic!(),
                Token::ClosingBracket { span } => break,
                Token::ClosingParen { span } => panic!(),
                _ => {
                    inside.push(token);
                }
            }
        }

        Ok(Delimiter { tokens: inside })
    }
    pub fn brace_delimiter(&mut self) -> Result<Delimiter<'{', '}'>, ()> {
        let mut inside = Vec::new();
        while let Ok(token) = self.next_token() {
            match token {
                Token::ClosingBrace { span } => break,
                Token::ClosingBracket { span } => panic!(),
                Token::ClosingParen { span } => panic!(),
                _ => {
                    inside.push(token);
                }
            }
        }

        Ok(Delimiter { tokens: inside })
    }
}
