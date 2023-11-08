use std::slice;

use crate::error::{ExpectedNode, ParseError};
use casey::upper;

use parmesan_common::{Span, Spanned};
gen_token!(
    Integer,
    Float,
    Ident,
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
    #[lexeme = "-="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    MinusEq,
    #[lexeme = "*"]
    #[group(BinaryOperator)]
    Star,
    #[lexeme = "**"]
    #[group(BinaryOperator)]
    StarStar,
    #[lexeme = "**="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    StarStarEq,
    #[lexeme = "*="]
    #[group(BinaryOperator, MutableBinaryOperator)]
    StarEq,
    #[lexeme = "/"]
    #[group(BinaryOperator)]
    Slash,
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
    #[group(BinaryOperator)]
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
    // keywords
    True,
    False,
    If,
    Else,
    FnKeyword,
    Return,
    Let,
    Mut
);

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
        Token::Star(Star {
            lexeme: "*",
            span: Span {
                src_start: 37,
                src_end: 38,
                line_start: 37,
                line: 0,
            },
        }),
        Token::StarStar(StarStar {
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
use parmesan_dev_macros::gen_token;

use crate::{lexer::Lexer, traits::Node};
pub fn tokens_by_line<'a>(tokens: &Vec<Token<'a>>) -> Vec<&'a [Token<'a>]> {
    let mut lines: Vec<&[Token<'a>]> = Vec::new();
    let mut line_start: *const Token<'a> = tokens.get(0).unwrap();

    let mut len = 0;
    let mut current_line = 0;
    let mut prev_start = 0;
    let mut prev_line = 0;

    for token in tokens {
        let span = token.span();
        assert!(
            span.src_start >= prev_start && span.line >= prev_line,
            "Tokens must be in sequential order"
        );

        if token.span().line != current_line {
            current_line = token.span().line;
            lines.push(unsafe { slice::from_raw_parts(line_start, len) });

            len = 0;
            line_start = token;
        }

        len += 1;
        prev_start = span.src_start;
        prev_line = span.line;
    }

    lines.push(unsafe { slice::from_raw_parts(line_start, len) });

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
        tokens_by_line(&tokens).get(0).unwrap(),
        &[line1.clone(), item2.clone()]
    );
    assert_eq!(tokens_by_line(&tokens).get(1).unwrap(), &[line2.clone()]);
    assert_eq!(tokens_by_line(&tokens).get(2).unwrap(), &[line3.clone()])
}
impl Spanned for Token<'_> {
    fn span(&self) -> Span {
        self.span()
    }
}
