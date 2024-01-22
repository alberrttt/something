use crate::{
    ast::error::{ErrorKind, ExpectedNode},
    ast::prelude::*,
};
use parm_common::{Span, Spanned};
use std::marker::PhantomData;
struct Dook<'a>(PhantomData<&'a ()>);
impl<'a> Spanned for Dook<'a> {
    fn span(&self) -> Span {
        todo!()
    }
}
impl<'a> Node<'a> for Dook<'a> {
    fn parse(parse_stream: &mut crate::ast::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let peeked = parse_stream.peek()?;
        if let Token::Integer(peeked) = peeked {
            let _tmp: Result<Integer<'a>, ErrorKind<'a>> = Ok(peeked.clone());
            parse_stream.advance()?;
            Ok(Dook(PhantomData::default()))
        } else {
            ParseError::err(
                ErrorKind::ExpectedNode(ExpectedNode {
                    got: peeked.lexeme(),
                    expected: "Integer",
                    location: peeked.span(),
                }),
                parse_stream.tokens,
                parse_stream.src_file,
            )
        }
    }
}
pub static COMPILER_IDENT: Identifier<'static> = Identifier {
    lexeme: "COMPILER IDENT",
    span: Span {
        src_start: 123321456654,
        src_end: 123321456654,
        line_start: 123321456654,
        line: 123321456654,
    },
};
gen_token!(
    Integer,
    Float,
    Identifier,
    // syntax
    #[lexeme = "("]
    LParen,
    #[lexeme = ")"]
    RParen,
    #[lexeme = "{"]
    LBrace,
    #[lexeme = "}"]
    RBrace,
    #[lexeme = "["]
    LBracket,
    #[lexeme = "]"]
    RBracket,
    #[lexeme = "."]
    Dot,
    #[lexeme = ","]
    Comma,
    #[lexeme = ":"]
    Colon,
    #[lexeme = "::"]
    ColonColon,
    #[lexeme = ";"]
    SemiColon,
    #[lexeme = "+"]
    #[group(BinaryOperator)]
    Plus,
    #[lexeme = "+="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    PlusEq,
    #[lexeme = "-"]
    #[group(BinaryOperator)]
    Minus,
    #[lexeme = "->"]
    RightArrow,
    #[lexeme = "-="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    DashEq,
    #[lexeme = "*"]
    #[group(BinaryOperator)]
    Asterisk,
    #[lexeme = "**"]
    #[group(BinaryOperator)]
    AsteriskAsterisk,
    #[lexeme = "**="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    StarStarEq,
    #[lexeme = "*="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    StarEq,
    #[lexeme = "/"]
    #[group(BinaryOperator)]
    Slash,
    #[lexeme = "//"]
    SlashSlash,
    #[lexeme = "/="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    SlashEq,
    #[lexeme = "%"]
    #[group(BinaryOperator)]
    Percent,
    #[lexeme = "%="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    PercentEq,
    #[lexeme = "="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    Eq,
    #[lexeme = "=="]
    #[group(BinaryOperator)]
    EqEq,
    #[lexeme = ">"]
    #[group(BinaryOperator)]
    Greater,
    #[lexeme = ">="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    GreaterEq,
    #[lexeme = "<"]
    #[group(BinaryOperator)]
    Less,
    #[lexeme = "<="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    LessEq,
    #[lexeme = "!"]
    Bang,
    #[lexeme = "!="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    BangEq,
    #[lexeme = "&"]
    #[group(BinaryOperator)]
    Amper,
    #[lexeme = "&="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    AmperEq,
    #[lexeme = "&&"]
    #[group(BinaryOperator)]
    AmperAmper,
    #[lexeme = "&&="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    AmperAmperEq,
    #[lexeme = "|"]
    #[group(BinaryOperator)]
    Pipe,
    #[lexeme = "|="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    PipeEq,
    #[lexeme = "||"]
    #[group(BinaryOperator)]
    PipePipe,
    #[lexeme = "||="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    PipePipeEq,
    #[lexeme = "^"]
    #[group(BinaryOperator)]
    Caret,
    #[lexeme = "^="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    CaretEq,
    #[lexeme = "#"]
    Octothorpe,
    #[lexeme = "'"]
    SingleQuote,
    // keywords
    True,
    False,
    If,
    Else,
    FnKeyword,
    Return,
    Let,
    Mut,
    Use,
    Loop,
    While,
    For,
    In,
    Break,
    Continue,
    StructKeyword,
    Enum,
    Impl,
    TypeKeyword,
    Trait,
    LSelf,
    USelf,
    Pub,
    #[no_impl]
    StringLiteral,
);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StringLiteral<'a> {
    pub lexeme: &'a str,
    pub span: Span,
}
impl<'a> TreeDisplay for StringLiteral<'a> {
    fn tree(&self) -> Tree {
        Tree::new("String Literal").lexeme(self.lexeme)
    }
}
impl<'a> Node<'a> for StringLiteral<'a> {
    fn parse(parse_stream: &mut ParseStream<'a>) -> Result<Self, Box<ParseError<'a>>>
    where
        Self: Sized,
    {
        let peeked = match parse_stream.peek() {
            Ok(peeked) => peeked,
            Err(err) => {
                return ParseError::err(
                    ErrorKind::EndOfTokens(EndOfTokens {
                        expected: Some("string literal"),
                    }),
                    parse_stream.tokens,
                    parse_stream.src_file,
                )
            }
        };
        if let Token::StringLiteral(peeked) = peeked {
            let tmp = Ok(peeked.clone());
            parse_stream.advance()?;
            tmp
        } else {
            ParseError::err(
                ErrorKind::ExpectedToken(ExpectedToken {
                    got: peeked.to_owned(),
                    expected: Token::StringLiteral(Self::default()),
                    location: parse_stream.current,
                }),
                parse_stream.tokens,
                parse_stream.src_file,
            )
        }
    }
}
impl<'a> Spanned for StringLiteral<'a> {
    fn span(&self) -> Span {
        self.span
    }
}
impl<'a> StringLiteral<'a> {
    pub fn spanless_eq(&self, other: &Self) -> bool {
        other.lexeme == self.lexeme
    }
}
impl<'a> BinaryOperator<'a> {
    pub fn is_equality(&self) -> bool {
        matches!(
            self,
            BinaryOperator::EqEq(_)
                | BinaryOperator::BangEq(_)
                | BinaryOperator::LessEq(_)
                | BinaryOperator::GreaterEq(_)
        )
    }
    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            BinaryOperator::Less(_)
                | BinaryOperator::Greater(_)
                | BinaryOperator::LessEq(_)
                | BinaryOperator::GreaterEq(_)
        )
    }
    pub fn is_boolean_logic_operator(&self) -> bool {
        matches!(
            self,
            BinaryOperator::PipePipe(_) | BinaryOperator::AmperAmper(_)
        )
    }
}
#[test]
fn test() {
    let mut lexer = Lexer::from(")( ][, || ||= && != !! &&= &= |= **= * ** %= += +");
    let cases = vec![
        Token::RParen(RParen {
            lexeme: ")",
            span: Span {
                src_start: 0,
                src_end: 1,
                line_start: 0,
                line: 0,
            },
        }),
        Token::LParen(LParen {
            lexeme: "(",
            span: Span {
                src_start: 1,
                src_end: 2,
                line_start: 1,
                line: 0,
            },
        }),
        Token::RBracket(RBracket {
            lexeme: "]",
            span: Span {
                src_start: 3,
                src_end: 4,
                line_start: 3,
                line: 0,
            },
        }),
        Token::LBracket(LBracket {
            lexeme: "[",
            span: Span {
                src_start: 4,
                src_end: 5,
                line_start: 4,
                line: 0,
            },
        }),
        Token::Comma(Comma {
            lexeme: ",",
            span: Span {
                src_start: 5,
                src_end: 6,
                line_start: 5,
                line: 0,
            },
        }),
        Token::PipePipe(PipePipe {
            lexeme: "||",
            span: Span {
                src_start: 7,
                src_end: 9,
                line_start: 7,
                line: 0,
            },
        }),
        Token::PipePipeEq(PipePipeEq {
            lexeme: "||=",
            span: Span {
                src_start: 10,
                src_end: 13,
                line_start: 10,
                line: 0,
            },
        }),
        Token::AmperAmper(AmperAmper {
            lexeme: "&&",
            span: Span {
                src_start: 14,
                src_end: 16,
                line_start: 14,
                line: 0,
            },
        }),
        Token::BangEq(BangEq {
            lexeme: "!=",
            span: Span {
                src_start: 17,
                src_end: 19,
                line_start: 17,
                line: 0,
            },
        }),
        Token::Bang(Bang {
            lexeme: "!",
            span: Span {
                src_start: 20,
                src_end: 21,
                line_start: 20,
                line: 0,
            },
        }),
        Token::Bang(Bang {
            lexeme: "!",
            span: Span {
                src_start: 21,
                src_end: 22,
                line_start: 21,
                line: 0,
            },
        }),
        Token::AmperAmperEq(AmperAmperEq {
            lexeme: "&&=",
            span: Span {
                src_start: 23,
                src_end: 26,
                line_start: 23,
                line: 0,
            },
        }),
        Token::AmperEq(AmperEq {
            lexeme: "&=",
            span: Span {
                src_start: 27,
                src_end: 29,
                line_start: 27,
                line: 0,
            },
        }),
        Token::PipeEq(PipeEq {
            lexeme: "|=",
            span: Span {
                src_start: 30,
                src_end: 32,
                line_start: 30,
                line: 0,
            },
        }),
        Token::StarStarEq(StarStarEq {
            lexeme: "**=",
            span: Span {
                src_start: 33,
                src_end: 36,
                line_start: 33,
                line: 0,
            },
        }),
        Token::Asterisk(Asterisk {
            lexeme: "*",
            span: Span {
                src_start: 37,
                src_end: 38,
                line_start: 37,
                line: 0,
            },
        }),
        Token::AsteriskAsterisk(AsteriskAsterisk {
            lexeme: "**",
            span: Span {
                src_start: 39,
                src_end: 41,
                line_start: 39,
                line: 0,
            },
        }),
        Token::PercentEq(PercentEq {
            lexeme: "%=",
            span: Span {
                src_start: 42,
                src_end: 44,
                line_start: 42,
                line: 0,
            },
        }),
        Token::PlusEq(PlusEq {
            lexeme: "+=",
            span: Span {
                src_start: 45,
                src_end: 47,
                line_start: 45,
                line: 0,
            },
        }),
        Token::Plus(Plus {
            lexeme: "+",
            span: Span {
                src_start: 48,
                src_end: 49,
                line_start: 48,
                line: 0,
            },
        }),
    ];
    for (input, against) in lexer.lex().iter().zip(cases.iter()) {
        assert_eq!(input, against)
    }
}
use parm_dev_macros::gen_token;

use crate::ast::{lexer::Lexer, traits::Node};
pub fn tokens_by_line<'a, 'b: 'a>(tokens: &'b [Token<'a>]) -> Vec<(usize, &'b [Token<'a>])> {
    let mut lines: Vec<(usize, &[Token<'a>])> = Vec::new();
    let mut line_start = 0;

    let mut len = 0;
    let mut current_line = 0;
    let mut prev_start = 0;
    let mut prev_line = 0;

    for (idx, token) in tokens.iter().enumerate() {
        let span = token.span();

        assert!(
            span.src_start >= prev_start && span.line >= prev_line,
            "Tokens must be in spanless_equential order"
        );

        if token.span().line != current_line {
            lines.push((current_line, &tokens[line_start..line_start + len]));

            current_line = token.span().line;
            len = 0;
            line_start = idx;
        }

        len += 1;
        prev_start = span.src_start;
        prev_line = span.line;
    }

    lines.push((prev_line, &tokens[line_start..line_start + len]));

    lines
}
#[test]
fn test_tokens_by_line() {
    let mut lexer = Lexer::from("line1 item2\nline2\nline3");
    let tokens = lexer.lex();
    let line1 = tokens.get(0).unwrap();
    let item2 = tokens.get(1).unwrap();
    let line2 = tokens.get(2).unwrap();
    let line3 = tokens.get(3).unwrap();

    assert_eq!(
        tokens_by_line(&tokens).first().unwrap().1,
        &[line1.clone(), item2.clone()]
    );
    assert_eq!(tokens_by_line(&tokens).get(1).unwrap().1, &[line2.clone()]);
    assert_eq!(tokens_by_line(&tokens).get(2).unwrap().1, &[line3.clone()]);
}
