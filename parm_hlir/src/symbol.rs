use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use parm_ast::parser::nodes::statement::use_stmt::{
    FunctionDeclaration, LetStatement, Param, StructDeclaration,
};

use crate::ty::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol<'a, 'b> {
    pub inner: Rc<RefCell<InnerSymbol<'a, 'b>>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolRef {
    pub id: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub struct InnerSymbol<'a, 'b> {
    pub id: usize,
    pub declaration: SymbolDeclaration<'a, 'b>,
    pub ty: Type<'a, 'b>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolDeclaration<'a, 'b> {
    Function(&'b FunctionDeclaration<'a>),
    Struct(&'b StructDeclaration<'a>),
    LetStatement(&'b LetStatement<'a>),
    Param(&'b Param<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolArena<'a, 'b> {
    pub symbols: Vec<Symbol<'a, 'b>>,
}

impl<'a, 'b> SymbolArena<'a, 'b> {
    pub fn from_function_declaration(&mut self, func: &'b FunctionDeclaration<'a>) {
        let id = self.symbols.len();
        let function = crate::function::Function {
            symbol: SymbolRef { id },
            return_ty: Type::Number,
            statements: vec![],
        };
        self.symbols.push(Symbol {
            inner: Rc::new(RefCell::new(InnerSymbol {
                id,
                declaration: SymbolDeclaration::Function(func),
                ty: Type::Function(Rc::new(function)),
            })),
        });
    }
}
