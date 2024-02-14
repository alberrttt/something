use std::{
    cell::{RefCell, UnsafeCell},
    marker::PhantomData,
    rc::Rc,
};

use parm_ast::{
    parser::nodes::statement::use_stmt::FunctionDeclaration, source_file::PreparsedSourceFile,
};

use crate::{
    function::Function,
    scope::{Scope, ScopeArena},
    statement::Statement,
    symbol::{self, InnerSymbol, Symbol, SymbolArena, SymbolDeclaration},
    ty::{function_ty::FunctionTy, Type, TypeArena, TypeRef},
};

pub struct Typechecker<'a, 'b> {
    pub source_file: &'b PreparsedSourceFile<'a>,
    pub scopes_arena: ScopeArena<'a, 'b>,
    pub symbols_arena: SymbolArena<'a, 'b>,
    pub types_arena: TypeArena<'a, 'b>,
    pub current_scope: usize,
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
        for symbol in &scope.symbols {
            let inner = symbol.inner.borrow();
            if inner.lexeme == lex {
                return Some(symbol.clone());
            }
        }
        None
    }
    pub fn new(src_file: &'b PreparsedSourceFile<'a>) -> Self {
        let mut tc = Self {
            source_file: src_file,
            symbols_arena: SymbolArena::default(),
            types_arena: TypeArena {
                types: Self::default_types(),
            },
            scopes_arena: ScopeArena::new(),
            current_scope: 0,
        };
        tc.symbols_arena.symbols.push(Symbol {
            inner: Rc::new(RefCell::new(InnerSymbol {
                id: 0,
                declaration: SymbolDeclaration::None,
                ty: TypeRef::new(0),
                lexeme: "<reserved for compiler>",
                tc: &tc,
            })),
        });
        tc
    }
    pub fn check_fn(&mut self, function: &'b FunctionDeclaration<'a>) -> Function<'a, 'b> {
        self.scopes_arena.push(Some(self.current_scope));
        let symbol_idx = self.symbols_arena.symbols.len();
        let ty_id = self.types_arena.types.len();
        let symbol = InnerSymbol {
            id: symbol_idx,
            declaration: SymbolDeclaration::Function(function),
            ty: TypeRef::new(ty_id),
            lexeme: function.name.lexeme,
            tc: self,
        };
        let symbol = Symbol {
            inner: Rc::new(RefCell::new(symbol)),
        };
        self.symbols_arena.symbols.push(symbol.clone());
        let ty = Rc::new(FunctionTy {
            symbol: symbol.clone(),
            return_ty: TypeRef::new(0),
        });
        self.types_arena.types.push(Type::Function(ty.clone()));
        let mut statements = vec![];
        for statement in &function.body.statements.inner {
            statements.push(Statement::from_ast(self, statement))
        }
        let function = Function {
            symbol: symbol.clone(),
            statements,
        };

        function
    }
}
