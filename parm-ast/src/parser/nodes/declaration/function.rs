use crate::prelude::*;
use parm_common::Spanned;
use parm_dev_macros::Spanned;
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Function<'a> {
    pub fn_tkn: FnKeyword<'a>,
    pub name: Identifier<'a>,
    pub params: Paren<'a, Punctuated<Identifier<'a>, Comma<'a>>>,
    pub body: Brace<'a, Vec<Item<'a>>>,
    pub arrow: RightArrow<'a>,
    pub ret_type: TypeExpression<'a>,
}
impl<'a> Node<'a> for Function<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let fn_token = parser.step(|parser| FnKeyword::parse(parser).clone())?;
        let name = Identifier::parse(parser)?;
        let params: Paren<'_, Punctuated<Identifier<'_>, Comma<'_>>> =
            parser.step(|parser| Paren::parse_manual(parser, Punctuated::parse_terminated))?;
        let body = parser.step(|parser| {
            Brace::parse_manual(parser, |parser| {
                let mut body = Vec::new();
                loop {
                    if parser.at_end() {
                        break;
                    }
                    match Item::parse(parser) {
                        Ok(res) => body.push(res),
                        Err(err) => {
                            eprintln!("{}", err);
                            break;
                        }
                    };
                }
                Ok(body)
            })
        })?;
        let arrow = parser.step(|parser| RightArrow::parse(parser).clone())?;
        let ret_type = parser.step(|parser| TypeExpression::parse(parser).clone())?;
        Ok(Self {
            fn_tkn: fn_token,
            name,
            params,
            body,
            arrow,
            ret_type,
        })
    }
}

#[test]
fn test_fn() {
    let input = "fn foo(hello) {
        let x = 5;
        let y = 6;
        x + 2;
    } -> wassup::bejing<foo,bar>::icecream";
    let mut parser = Parser::new(input);
    let result = Function::parse(&mut parser.stream()).unwrap();
    dbg!(result);
}
