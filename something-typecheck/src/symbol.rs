use std::{collections::HashSet, fmt::Debug};

use something_ast::tokenizer::prelude::Ident;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    pub symbol_type: Type,
    pub name: String,
}
impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "symbol {} <{:?}>", self.name, self.symbol_type)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub enum Type {
    Int,
    Float,
    Bool,
    Void,
    /// TODO, IT PROLLS NEEDS A SUB SYMBOL TABLE
    Function(FnType),
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FnType {
    pub params: Vec<(Type, Ident)>,
    pub return_type: Box<Type>,
}
impl Debug for FnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn({:?}) -> {:?}", self.params, self.return_type)
    }
}
#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: HashSet<Symbol>,
}
