use crate::prelude::*;
use std::{cell::UnsafeCell, path::PathBuf};

use crate::{lexer::Lexer, parser::Parser, prelude::Node};

#[derive(Debug, PartialEq)]
pub struct PreparsedSourceFile<'a> {
    pub path: PathBuf,
    pub src: &'a str,
    pub lexer: Lexer<'a>,
    pub parser: Parser<'a>,
}
impl<'a> PreparsedSourceFile<'a> {
    pub fn new(path: PathBuf, src: &'a str) -> Self {
        let mut lexer = Lexer::from(src);
        let tokens = lexer.lex();
        let parser = Parser {
            src,
            tokens,
            current: 0,
        };

        Self {
            path,
            src,
            lexer,
            parser,
        }
    }
}
impl<'a> PreparsedSourceFile<'a> {
    pub fn parse(self) -> (SourceFile<'a>, Vec<ParseError<'a>>) {
        let src_file: &mut UnsafeCell<PreparsedSourceFile<'_>> =
            Box::leak(Box::new(UnsafeCell::new(self)));

        let mut stream = ParseStream {
            tokens: &unsafe { &*src_file.get() }.parser.tokens,
            current: 0,
            src_file,
            panic: false,
            attributes: Default::default(),
            errors: Default::default(),
        };
        let (ast, errors) =
            <Vec<Item<'a>> as Node<'a, (Vec<Item<'a>>, Vec<ParseError<'a>>)>>::parse(&mut stream);
        (
            SourceFile {
                preparsed: src_file,
                ast,
            },
            errors,
        )
    }
}
#[derive(Debug)]
pub struct SourceFile<'a> {
    pub preparsed: &'a UnsafeCell<PreparsedSourceFile<'a>>,
    pub ast: Vec<Item<'a>>,
}
