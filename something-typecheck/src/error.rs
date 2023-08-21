use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;

pub struct TypeError {
    pub surrounding: Option<TokenStream>,
    pub kind: TypeErrorKind,
    pub backtrace: Option<Backtrace>,
}

impl Debug for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeError")
            .field("surrounding", &self.surrounding)
            .field("kind", &self.kind)
            .finish()
    }
}
impl Clone for TypeError {
    fn clone(&self) -> Self {
        Self {
            surrounding: self.surrounding.clone(),
            kind: self.kind.clone(),
            backtrace: match &self.backtrace {
                Some(backtrace) => {
                    panic!();
                }
                None => None,
            },
        }
    }
}
#[allow(non_snake_case)]
impl TypeError {
    pub fn Generic(msg: impl Into<String>) -> Self {
        Self::create(
            &Token::Eof(Default::default()),
            TypeErrorKind::Generic(msg.into()),
        )
    }
}
impl TypeError {
    fn create(surrounding_tokens: &dyn AppendTokens, kind: TypeErrorKind) -> Self {
        let mut tokenstream = TokenStream::default();
        surrounding_tokens.append_tokens(&mut tokenstream);
        Self {
            surrounding: Some(tokenstream),
            kind,

            backtrace: Some(Backtrace::capture()),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeErrorKind {
    Generic(String),
}
use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt::Debug;
use std::{any, default};
#[derive(Debug, Clone)]
pub struct ExpectedAst {
    pub ast: any::TypeId,
}
#[derive(Debug, Clone)]
pub struct ExpectedToken {
    pub expected: Token,
    pub at: usize, // <- an index to `surrounding`
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error ")?;
        let surrounding = self.surrounding.as_ref().unwrap();
        match &self.backtrace {
            Some(b) => {
                if std::env::var("ERR_BACKTRACE").unwrap_or_default() == "1" {
                    match &self.backtrace {
                        Some(backtrace) => {
                            writeln!(f, "\n{}", backtrace)?;
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
        use TypeErrorKind::*;
        match &self.kind {
            Generic(string) => {
                write!(f, "{}", string)
            }
        }
    }
}
impl std::error::Error for TypeError {}
