use std::{backtrace, fmt::Display};

use crate::tokenizer::prelude::*;
use something_dev_tools::{item_name, ParseTokensDisplay};
pub mod block;
pub mod call;
pub mod if_expr;
pub mod precedence;
#[derive(Clone, ParseTokensDisplay)]
pub enum Expression {
    Lit(Literal),
    Binary(Binary),
    Call(Call),
    Ident(Ident),
    Grouping(Parentheses<Box<Expression>>),
    If(if_expr::If),
    Block(block::Block),
}
impl AppendTokens for Expression {
    fn append_tokens(&self, tokens: &mut Tokens)
    where
        Self: Sized,
    {
        match self {
            Expression::Lit(lit) => lit.append_tokens(tokens),
            Expression::Binary(binary) => binary.append_tokens(tokens),
            Expression::Call(_) => todo!(),
            Expression::Ident(_) => todo!(),
            Expression::Grouping(_) => todo!(),
            Expression::If(_) => todo!(),
            Expression::Block(_) => todo!(),
        }
    }
}
impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lit(arg0) => write!(f, "{:#?}", arg0),
            Self::Binary(arg0) => write!(f, "{:#?}", arg0),
            Self::Call(arg0) => write!(f, "{:#?}", arg0),
            Self::Ident(arg0) => write!(f, "{:#?}", arg0),
            Self::Grouping(arg0) => write!(f, "{:?}", arg0),
            Self::If(arg0) => write!(f, "{:#?}", arg0),
            Self::Block(arg0) => write!(f, "{:#?}", arg0),
        }
    }
}
item_name!(Expression, "expression");
use crate::ast::delimiter::Parentheses;

pub use self::call::*;
impl Parse for Expression {
    fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
        let tmp = match input.peek() {
            Some(token) => token.clone(),
            None => {
                return Err(ParseError::Generic(format!("end of file")));
            }
        };
        parse_expr(
            match tmp {
                Token::If(if_token) => {
                    let tmp = if_expr::If::parse(input).unwrap();
                    let tmp = Ok(Expression::If(tmp));

                    tmp
                }
                Token::Braces(_) => {
                    let tmp = block::Block::parse(input).unwrap();

                    Ok(Expression::Block(tmp))
                }
                Token::Lit(lit) => {
                    input.advance();
                    Ok(Self::Lit(lit))
                }
                Token::Ident(ident) => {
                    if let Some(Token::Parentheses(_)) = input.peek1() {
                        Ok(Expression::Call(Call::parse(input)?))
                    } else {
                        input.advance();
                        Ok(Expression::Ident(ident))
                    }
                }
                x => Err(ParseError::ExpectedAst("Expression".into(), x.to_string())),
            },
            input,
        )
    }
}
impl Parse for Box<Expression> {
    fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
        Ok(Box::new(Expression::parse(input)?))
    }
}
fn parse_expr(
    left: Result<Expression, ParseError>,
    input: &mut Tokens,
) -> Result<Expression, ParseError> {
    let left = left?;
    let token = match input.peek() {
        Some(token) => token.clone(),
        _ => {
            return Ok(left);
        }
    };
    match token {
        Token::Plus(_)
        | Token::Minus(_)
        | Token::Greater(_)
        | Token::Less(_)
        | Token::GreaterEqual(_)
        | Token::LessEqual(_)
        | Token::EqualEqual(_) => match Operator::parse(input) {
            Ok(operator) => {
                let right = match Expression::parse(input) {
                    Ok(right) => right,
                    Err(err) => {
                        println!    ("{}", err);
                        panic!()
                    }
                };
                Ok(Expression::Binary(Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }))
            }

            Err(_) => Ok(left),
        },
        Token::Equal(_)
        | Token::PlusEqual(_)
        | Token::MinusEqual(_)
        | Token::StarEqual(_)
        | Token::SlashEqual(_) => match Operator::parse(input) {
            Ok(operator) => {
                let right = Expression::parse(input).expect("Expected Expression");
                Ok(Expression::Binary(Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }))
            }

            Err(_) => Ok(left),
        },
        Token::Star(_) | Token::Slash(_) => match Operator::parse(input) {
            Ok(operator) => {
                let right = Expression::parse(input).expect("Expected Expression");
                parse_expr(
                    Ok(Expression::Binary(Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    })),
                    input,
                )
            }

            Err(_) => Ok(left),
        },

        token => Ok(left),
    }
}
#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}
impl AppendTokens for Binary {
    fn append_tokens(&self, tokens: &mut Tokens)
    where
        Self: Sized,
    {
        self.left.append_tokens(tokens);
        self.operator.append_tokens(tokens);
        self.right.append_tokens(tokens);
    }
}
impl ParsingDisplay for Binary {
    fn display(&self) -> String
    where
        Self: Sized,
    {
        format!(
            "{} {} {}",
            self.left.display(),
            self.operator.display(),
            self.right.display()
        )
    }

    fn placeholder() -> String
    where
        Self: Sized,
    {
        format!(
            "{} {} {}",
            Expression::placeholder(),
            Operator::placeholder(),
            Expression::placeholder()
        )
    }
}
impl From<Binary> for Expression {
    fn from(binary: Binary) -> Self {
        Self::Binary(binary)
    }
}

impl Parse for Binary {
    fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
        let expr = Expression::parse(input)?;
        todo!();
    }
}

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct Operator {
    pub kind: OperatorKind,
    pub token: Token,
}
impl AppendTokens for Operator {
    fn append_tokens(&self, tokens: &mut Tokens)
    where
        Self: Sized,
    {
        tokens.push(self.token.clone());
    }
}
impl Parse for Operator {
    fn parse(input: &mut Tokens) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let token = input.peek().cloned();
        Ok(Self {
            kind: input.parse()?,
            token: token.unwrap(),
        })
    }
}
#[derive(Debug, Clone)]
pub enum OperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    EqualEqual,
    NotEqual,
    PlusEqual,
    MinusEqual,
    MultiplyEqual,
    DivideEqual,
}

impl ParsingDisplay for OperatorKind {
    fn display(&self) -> String
    where
        Self: Sized,
    {
        match self {
            Self::Plus => "+".into(),
            Self::Minus => "-".into(),
            Self::Multiply => "*".into(),
            Self::Divide => "/".into(),
            Self::EqualEqual => "==".into(),
            Self::Equal => "=".into(),
            Self::Greater => ">".into(),
            Self::Less => "<".into(),
            Self::GreaterEqual => ">=".into(),
            Self::LessEqual => "<=".into(),
            Self::PlusEqual => "+=".into(),
            Self::MinusEqual => "-=".into(),
            Self::MultiplyEqual => "*=".into(),
            Self::DivideEqual => "/=".into(),
            Self::NotEqual => "!=".into(),
        }
    }
    fn placeholder() -> String
    where
        Self: Sized,
    {
        "<operator>".into()
    }
}
impl Parse for OperatorKind {
    fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
        match input.advance() {
            Some(token) => Ok(match token {
                Token::Plus(_) => Self::Plus,
                Token::Minus(_) => Self::Minus,
                Token::Star(_) => Self::Multiply,
                Token::Slash(_) => Self::Divide,
                Token::EqualEqual(_) => Self::EqualEqual,
                Token::Equal(_) => Self::Equal,
                Token::Greater(_) => Self::Greater,
                Token::Less(_) => Self::Less,
                Token::GreaterEqual(_) => Self::GreaterEqual,
                Token::LessEqual(_) => Self::LessEqual,
                Token::PlusEqual(_) => Self::PlusEqual,
                Token::MinusEqual(_) => Self::MinusEqual,
                Token::StarEqual(_) => Self::MultiplyEqual,
                Token::SlashEqual(_) => Self::DivideEqual,
                Token::BangEqual(_) => Self::NotEqual,
                _ => {
                    return Err(ParseError::Generic(format!(
                        "Expected Operator, got {:?}",
                        token.clone()
                    )));
                }
            }),
            _ => Err(ParseError::Generic(format!(
                "Expected Operator, got {:?}",
                input.advance().clone()
            ))),
        }
    }
}
