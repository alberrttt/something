use something_ast::prelude::FunctionDeclaration;

#[derive(Debug, Clone)]
pub enum Symbol {
    Fn { name: String },
}

impl From<&FunctionDeclaration> for Symbol {
    fn from(value: &FunctionDeclaration) -> Self {
        Self::Fn {
            name: value.name.to_string(),
        }
    }
}
