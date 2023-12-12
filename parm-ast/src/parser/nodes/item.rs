use crate::prelude::*;
use parm_common::Spanned;
use parm_dev_macros::Spanned;

use super::comment::Comment;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub enum Item<'a> {
    Variable(Variable<'a>),
    Function(Function<'a>),
    Statement(Statement<'a>),
    Use(UseStatement<'a>),
    Return(ReturnStatement<'a>),
    Comment(Comment<'a>),
}

impl<'a> Node<'a> for Item<'a> {
    fn parse(parser: &mut super::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let attribute = parser.step(|parser| Attribute::parse(parser).clone());
        if let Ok(attribute) = attribute {
            parser.attributes.push(attribute);
        }
        let peeked = parser.peek()?;

        match peeked {
            Token::SlashSlash(slash_slash) => {
                return Ok(Self::Comment(Comment::parse(parser)?));
            }
            Token::Let(_) => match <Variable as Node>::parse(parser) {
                Ok(ok) => return Ok(Item::Variable(ok)),
                Err(err) => {
                    parser.panic = true;
                    return Err(err);
                }
            },

            Token::FnKeyword(_) => {
                let func: Function = <Function as Node>::parse(parser)?;
                return Ok(Item::Function(func));
            }
            Token::Return(_) => {
                let ret: ReturnStatement = <ReturnStatement as Node>::parse(parser)?;
                return Ok(Item::Return(ret));
            }
            Token::Use(_) => {
                let use_stmt: UseStatement = <UseStatement as Node>::parse(parser)?;
                return Ok(Item::Use(use_stmt));
            }
            _ => {
                let expr = parser.step(Expression::parse);
                match expr {
                    Ok(expr) => {
                        return Ok(Item::Statement(Statement::with_expression(parser, expr)));
                    }
                    Err(_err) => {}
                }
            }
        }
        Err(ParseError::new(
            crate::error::ErrorKind::ExpectedNode(crate::error::ExpectedNode {
                got: parser.peek()?.lexeme(),
                expected: "an item",
                location: parser.current,
            }),
            parser.tokens,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct ReturnStatement<'a> {
    pub return_tkn: Return<'a>,
    pub expr: Expression<'a>,
    pub semi: SemiColon<'a>,
}
impl<'a> Node<'a> for ReturnStatement<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let return_tkn = parser.step(|parser| Return::parse(parser).clone())?;
        let expr = parser.step(|parser| Expression::parse(parser).clone())?;
        let semi = parser.step(|parser| SemiColon::parse(parser).clone())?;
        Ok(Self {
            return_tkn,
            expr,
            semi,
        })
    }
}
// #[test]
// fn test_var() {
//     let pre = PreparsedSourceFile::new("test".into(), "let x = 1;");
//     let mut parser = Parser::new("let x = 1;");
//     let var: Item = <Item as Node>::parse(&mut parser.stream()).unwrap();
//     dbg!(var);
// }

// #[test]
// fn test_fn() {
//     let mut parser = Parser::new("fn x() {}");
//     let var: Item = <Item as Node>::parse(&mut parser.stream()).unwrap();
//     dbg!(var);
// }
