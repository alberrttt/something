use std::{collections::HashMap, fmt::Error};

use parm_ast::{error::display_annotations, lexer::token::Token, source_file::PreparsedSourceFile};

use crate::prelude::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeError<'a, 'b: 'a> {
    pub kind: TypeErrorKind<'a, 'b>,
    pub surrounding: &'b [Token<'a>],
    pub file: &'b PreparsedSourceFile<'a>,
}
impl<'a, 'b: 'a> TypeError<'a, 'b> {
    pub fn new(
        kind: TypeErrorKind<'a, 'b>,
        surrounding: &'b [Token<'a>],
        file: &'b PreparsedSourceFile<'a>,
    ) -> Self {
        Self {
            kind,
            surrounding,
            file,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeErrorKind<'a, 'b: 'a> {
    MismatchedTypes {
        expected: Type<'a, 'b>,
        found: Type<'a, 'b>,
    },
}

impl<'a, 'b: 'a> std::fmt::Display for TypeError<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = HashMap::new();

        let Ok((result, token)) = display_annotations(self.file, map) else {
            return Err(Error);
        };

        write!(f, "{}", result)
    }
}
