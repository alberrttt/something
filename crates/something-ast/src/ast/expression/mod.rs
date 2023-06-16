use crate::tokenizer::prelude::*;
use crate::{peek_matches, prelude::*};
use something_dev_tools::{item_name, ParseTokensDisplay};
use ParseResult;
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
    fn append_tokens(&self, tokens: &mut TokenStream)
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
    fn parse(input: &mut TokenStream) -> ParseResult<Self> {
        parse_expr(input)
    }
}
impl Parse for Box<Expression> {
    fn parse(input: &mut TokenStream) -> ParseResult<Self> {
        Ok(Box::new(Expression::parse(input)?))
    }
}

impl TokenStream {
    pub(in crate::ast::expression) fn expr_unit(
        &mut self,
        can_recover: bool,
    ) -> ParseResult<Expression> {
        match self.advance()?.clone() {
            Token::Lit(literal) => Ok(Expression::Lit(literal)),
            Token::Ident(ident) => {
                match Call::parse_with_ident(ident.clone(), self) {
                    Recoverable => {}
                    Ok(ok) => return Ok(Expression::Call(ok)),
                    Err(err) => return Err(err),
                };
                Ok(Expression::Ident(ident))
            }

            _ => {
                if can_recover {
                    Recoverable
                } else {
                    Err(ParseError::Generic(
                        concat!(
                            "
        Expected literal, got something else
    
        Error originated from: 
    
    ",
                            file!(),
                            ":",
                            line!(),
                            "\nFix this later pls"
                        )
                        .into(),
                    ))
                }
            }
        }
    }
    pub(in crate::ast::expression) fn term(
        &mut self,
        can_recover: bool,
    ) -> ParseResult<Expression> {
        let mut result = self.expr_unit(can_recover)?;
        while peek_matches!(self, Token::Star(_) | Token::Slash(_)) {
            let op = self.advance()?.clone();
            let right = self.expr_unit(false)?;
            result = Expression::Binary(Binary::from_token(result, op, right));
        }
        Ok(result)
    }
}
fn parse_expr(input: &mut TokenStream) -> ParseResult<Expression> {
    let mut result = input.term(true)?;
    while peek_matches!(input, Token::Plus(_) | Token::Minus(_)) {
        let op = input.advance()?.clone();
        let right = input.term(false)?;

        result = Expression::Binary(Binary::from_token(result, op, right));
    }
    Ok(result)
}
#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}
impl Binary {
    fn new(left: Expression, op: Operator, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        }
    }
    fn from_token(left: Expression, op: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator: Operator::from(op),
            right: Box::new(right),
        }
    }
}
impl AppendTokens for Binary {
    fn append_tokens(&self, tokens: &mut TokenStream)
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
            "{}{}{}",
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
    fn parse(input: &mut TokenStream) -> ParseResult<Self> {
        let _expr = Expression::parse(input)?;
        todo!();
    }
}

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct Operator {
    pub kind: OperatorKind,
    pub token: Token,
}
impl AppendTokens for Operator {
    fn append_tokens(&self, tokens: &mut TokenStream)
    where
        Self: Sized,
    {
        tokens.push(self.token.clone());
    }
}
impl Parse for Operator {
    fn parse(input: &mut TokenStream) -> ParseResult<Self>
    where
        Self: Sized,
    {
        let token = input.peek()?.clone();
        Ok(Self {
            kind: input.parse()?,
            token,
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
impl From<Token> for Operator {
    fn from(value: Token) -> Self {
        Self {
            kind: value.clone().into(),
            token: value,
        }
    }
}
impl From<Token> for OperatorKind {
    fn from(token: Token) -> Self {
        match token {
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
            _ => panic!("Expected Operator, got {:?}", token),
        }
    }
}

impl Parse for OperatorKind {
    fn parse(input: &mut TokenStream) -> ParseResult<Self> {
        let token = input.advance()?.clone();
        Ok(Self::from(token))
    }
}
