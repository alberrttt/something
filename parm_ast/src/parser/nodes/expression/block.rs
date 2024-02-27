use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Tree, Spanned)]
pub struct Block<'a> {
    pub statements: Brace<'a, Vec<Statement<'a>>>,
}
impl<'a> Node<'a> for Block<'a> {
    fn parse(parse_stream: &mut ParseStream<'a>) -> Result<Self, Box<ParseError<'a>>>
    where
        Self: Sized,
    {
        let statements: Brace<'_, Vec<Statement<'_>>> = parse_stream.step(|parser| {
            Brace::parse_manual(parser, |parser| {
                let mut body: Vec<Statement<'_>> = Vec::new();
                loop {
                    if parser.at_end() {
                        break;
                    }
                    let peeked = parser.peek()?;
                    match Statement::parse(parser) {
                        Ok(res) => body.push(res),
                        Err(err) => {
                            eprintln!("{}", err);
                            match peeked {
                                Token::IfKw(_) => {
                                    loop {
                                        if let Ok(_) =
                                            parser.step(|parser| RBrace::parse(parser).clone())
                                        {
                                            break;
                                        } else if parser.advance().is_err() {
                                            break;
                                        }
                                    }
                                    parser.panic = false
                                }
                                Token::ForKw(_) => {
                                    loop {
                                        if let Ok(_) =
                                            parser.step(|parser| RBrace::parse(parser).clone())
                                        {
                                            break;
                                        } else if parser.advance().is_err() {
                                            dbg!(parser.tokens);
                                            break;
                                        }
                                    }
                                    parser.panic = false
                                }
                                Token::ReturnKw(_)
                                | Token::FnKw(_)
                                | Token::Identifier(_)
                                | Token::LetKw(_) => loop {
                                    if let Ok(_semicolon) =
                                        parser.step(|parser| SemiColon::parse(parser).clone())
                                    {
                                        break;
                                    } else if parser.advance().is_err() {
                                        break;
                                    }
                                    parser.panic = false
                                },

                                _ => {
                                    // lets just default to recovering at a Brace
                                    loop {
                                        if let Ok(_) =
                                            parser.step(|parser| RBrace::parse(parser).clone())
                                        {
                                            break;
                                        } else if parser.advance().is_err() {
                                            dbg!(parser.tokens);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    };
                }
                Ok(body)
            })
        })?;
        Ok(Self { statements })
    }
}
