use crate::prelude::*;
use parm_common::Spanned;
use parm_dev_macros::Spanned;

use super::nodes::expression::parse_unit;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub enum Item<'a> {
    Variable(Variable<'a>),
    Function(Function<'a>),
    Statement(Statement<'a>),
}
impl<'a> Node<'a> for Item<'a> {
    fn parse(parser: &mut super::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let peeked = parser.peek()?;
        match peeked {
            Token::Let(_) => {
                let var: Variable = <Variable as Node>::parse(parser)?;
                return Ok(Item::Variable(var));
            }
            Token::FnKeyword(_) => {
                let func: Function = <Function as Node>::parse(parser)?;
                return Ok(Item::Function(func));
            }

            _ => {
                let expr = parser.step(parse_unit);
                match expr {
                    Ok(expr) => {
                        return Ok(Item::Statement(Statement::with_expression(parser, expr)));
                    }
                    Err(err) => {}
                }
            }
        }

        Err(ParseError::new(
            crate::error::ErrorKind::ExpectedNode(crate::error::ExpectedNode {
                got: format!("{:?}", parser.peek()).leak(),
                expected: "Variable or Function",
            }),
            parser.tokens,
        ))
    }
}

#[test]
fn test_var() {
    let mut parser = Parser::new("let x = 1;");
    let var: Item = <Item as Node>::parse(&mut parser.stream()).unwrap();
    dbg!(var);
}

#[test]
fn test_fn() {
    let mut parser = Parser::new("fn x() {}");
    let var: Item = <Item as Node>::parse(&mut parser.stream()).unwrap();
    dbg!(var);
}
