use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    rc::Rc,
};

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
    Number,
    Bool,
    Void,
    /// TODO, IT PROLLS NEEDS A SUB SYMBOL TABLE
    Function(Box<FnSig>),
    /// This should never be a finalized type
    /// Meaning, if this type exists in the finalized symbol table, it's an error
    Unknown,
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unknown => write!(f, "!!unknown!!"),
            Type::Number => write!(f, "number"),
            Type::Bool => write!(f, "bool"),
            Type::Void => write!(f, "void"),
            Type::Function(sig) => {
                let params = sig
                    .params
                    .iter()
                    .map(|s| s.symbol_type.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "fn({}) -> {}", params, sig.return_type)
            }
        }
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FnSig {
    pub params: Vec<Rc<Symbol>>,
    pub return_type: Type,
}
impl Debug for FnSig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn({:?}) -> {:?}", self.params, self.return_type)
    }
}
#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: HashSet<Symbol>,
}
