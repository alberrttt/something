use std::collections::HashSet;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    pub symbol_type: Type,
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub enum Type {
    Int,
    Float,
    Bool,
    /// TODO, IT PROLLS NEEDS A SUB SYMBOL TABLE
    Function,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: HashSet<Symbol>,
}
