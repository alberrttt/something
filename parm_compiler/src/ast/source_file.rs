use crate::ast::prelude::*;
use std::{cell::UnsafeCell, path::PathBuf, rc::Rc};

use crate::ast::{lexer::Lexer, parser::Parser, prelude::Node};

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

#[derive(Debug, PartialEq, Clone)]
pub struct SourceFile<'a> {
    pub preparsed: &'a PreparsedSourceFile<'a>,
    pub ast: Vec<Item<'a>>,
}
