use parm_dev_macros::Spanned;

use super::expression::Expression;

pub mod expression_statement;
pub mod ret;
pub mod use_stmt;
pub mod variable;

use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    ExpressionWithSemi(ExpressionWithSemi<'a>),
    Item(Item<'a>),
    Let(LetStatement<'a>),
    Return(ReturnStatement<'a>),
}

#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct ExpressionWithSemi<'a> {
    pub expression: Expression<'a>,
    pub semi: SemiColon<'a>,
}
impl<'a> Node<'a> for Statement<'a> {
    fn parse(parse_stream: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let peek = parse_stream.peek()?;
        match peek {
            Token::Let(_) => {
                let let_stmt = parse_stream.step(LetStatement::parse)?;
                return Ok(Self::Let(let_stmt));
            }
            Token::Return(_) => {
                let return_stmt = parse_stream.step(ReturnStatement::parse)?;
                return Ok(Self::Return(return_stmt));
            }

            _ => {}
        }
        let expression = match parse_stream.step(Expression::parse) {
            Ok(expression) => expression,
            Err(err) => {
                if parse_stream.panic {
                    return Err(err);
                }
                return ParseError::err(
                    ErrorKind::ExpectedNode(ExpectedNode {
                        expected: "a statement",
                        got: peek.lexeme(),
                        location: peek.span(),
                    }),
                    err.surrounding,
                    err.file,
                );
            }
        };

        let semi = parse_stream.step(SemiColon::parse);
        match semi {
            Ok(semi) => Ok(Self::ExpressionWithSemi(ExpressionWithSemi {
                expression,
                semi,
            })),
            Err(err) => {
                if parse_stream.at_end() {
                    Ok(Self::Expression(expression))
                } else {
                    parse_stream.panic = true;
                    Err(err)
                }
            }
        }
    }
}
impl<'a> Statement<'a> {
    pub fn with_expression(parser: &mut ParseStream<'a>, expression: Expression<'a>) -> Self {
        let semi = parser.step(SemiColon::parse);
        match semi {
            Ok(semi) => Self::ExpressionWithSemi(ExpressionWithSemi { expression, semi }),
            Err(_) => Self::Expression(expression),
        }
    }
}
