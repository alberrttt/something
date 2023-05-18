use std::fmt::Display;

use something_ast::prelude::FunctionDeclaration;
use something_frontend_tokenizer::ident::Ident;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub name: Ident,
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ident<{}>", self.name)
    }
}
impl From<&FunctionDeclaration> for Symbol {
    fn from(value: &FunctionDeclaration) -> Self {
        Self {
            name: value.name.clone(),
        }
    }
}
impl From<&Ident> for Symbol {
    fn from(value: &Ident) -> Self {
        Self {
            name: value.clone(),
        }
    }
}
