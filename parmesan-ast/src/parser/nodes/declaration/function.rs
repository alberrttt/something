use crate::prelude::*;
use parmesan_common::Spanned;
use parmesan_dev_macros::Spanned;
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Function<'a> {
    pub fn_tkn: FnKeyword<'a>,
    pub name: Ident<'a>,
    pub params: Paren<'a, Punctuated<Ident<'a>, Comma<'a>>>,
    pub body: Brace<'a, Vec<Item<'a>>>,
    pub arrow: RightArrow<'a>,
}
impl<'a> Node<'a> for Function<'a> {
    fn parse(
        parser: &mut crate::parser::ParseStream<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let fn_token = parser.step(|parser| FnKeyword::parse(parser).clone())?;
        let name = Ident::parse(parser)?;
        let params: Paren<'_, Punctuated<Ident<'_>, Comma<'_>>> =
            parser.step(|parser| Paren::parse_manual(parser, Punctuated::parse_terminated))?;
        let body = parser.step(|parser| {
            Brace::parse_manual(parser, |parser| {
                let mut body = Vec::new();
                loop {
                    if parser.at_end() {
                        break;
                    }
                    let item = Item::parse(parser)?;
                    body.push(item);
                }
                Ok(body)
            })
        })?;
        let arrow = parser.step(|parser| RightArrow::parse(parser).clone())?;
        Ok(Self {
            fn_tkn: fn_token,
            name,
            params,
            body,
            arrow,
        })
    }
}

#[test]
fn test_fn() {
    let input = "fn foo(hello) {} ->";
    let mut parser = Parser::new(input);
    let result = Function::parse(&mut parser.stream()).unwrap();
    dbg!(result);
}
