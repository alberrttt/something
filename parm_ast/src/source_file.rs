use crate::prelude::*;
use std::{cell::UnsafeCell, path::PathBuf, rc::Rc};

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
    pub fn parse<'b: 'a>(&'b self) -> SourceFile<'b> {
        let mut stream = self.parser.stream(&self);
        let (ast, errors) = <Vec<Item> as Node<'_, (Vec<_>, Vec<_>)>>::parse(&mut stream);

        SourceFile {
            preparsed: &self,
            ast,
            errors,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SourceFile<'a> {
    pub preparsed: &'a PreparsedSourceFile<'a>,
    pub ast: Vec<Item<'a>>,
    pub errors: Vec<ParseError<'a>>,
}
