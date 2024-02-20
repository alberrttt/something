use std::{
    cell::{RefCell, UnsafeCell},
    marker::PhantomData,
    rc::Rc,
};

use parm_ast::{
    parser::nodes::statement::use_stmt::FunctionDeclaration,
    source_file::{PreparsedSourceFile, SourceFile},
};

use crate::{
    prelude::*,
    scope::{Scope, ScopeArena},
    statement::Statement,
};

use self::error::TypeError;

pub struct Typechecker<'a, 'b> {
    pub source_file: &'b SourceFile<'a>,
    pub scopes_arena: ScopeArena<'a, 'b>,
    pub current_scope: usize,
    pub errs: Vec<TypeError<'a, 'b>>,
    pub none_symbol: Symbol<'a, 'b>,
}

impl<'a, 'b> Typechecker<'a, 'b> {
    pub fn current_scope(&self) -> &Scope<'a, 'b> {
        &self.scopes_arena.scopes[self.current_scope]
    }
    pub fn mut_current_scope<'c>(&'c mut self) -> &'c mut Scope<'a, 'b> {
        &mut self.scopes_arena.scopes[self.current_scope]
    }
    /// O(n)
    pub fn get_symbol(&self, lex: &str) -> Option<Symbol<'a, 'b>> {
        let scope = self.current_scope();
        for (key, symbol) in &scope.symbols {
            let inner = symbol.inner.borrow();
            if inner.lexeme == lex {
                return Some(symbol.clone());
            }
        }
        None
    }
    pub fn new(src_file: &'b SourceFile<'a>) -> Self {
        let mut tc = Self {
            source_file: src_file,
            errs: vec![],
            scopes_arena: ScopeArena::new(),
            current_scope: 0,
            none_symbol: Symbol {
                inner: Rc::new(RefCell::new(InnerSymbol {
                    declaration: SymbolDeclaration::None,
                    ty: Type::Unknown,
                    lexeme: "",
                })),
            },
        };

        tc
    }
}
