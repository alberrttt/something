use core::panic;
use std::{error::Error, fmt::Display, rc::Rc};

use crate::{symbol::Symbol, traits::TypeCheck};
use colored::Colorize;
use something_ast::prelude::{return_type::ReturnType, *};
#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<(Type, Symbol)>,
    pub return_type: Type,
    pub fn_ast: Rc<FunctionDeclaration>,
}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.return_type == other.return_type
    }
}
impl From<&FunctionDeclaration> for Function {
    fn from(value: &FunctionDeclaration) -> Self {
        Self {
            params: value
                .params
                .1
                .iter()
                .map(|(ty, name)| (Type::from(ty.clone()), Symbol::from(name)))
                .collect(),
            return_type: Type::from(value.return_type.clone()),
            fn_ast: Rc::new(value.clone()),
        }
    }
}
impl From<&Rc<FunctionDeclaration>> for Function {
    fn from(value: &Rc<FunctionDeclaration>) -> Self {
        Self {
            params: value
                .params
                .1
                .iter()
                .map(|(ty, name)| (Type::from(ty.clone()), Symbol::from(name)))
                .collect(),
            return_type: Type::from(value.return_type.clone()),
            fn_ast: value.clone(),
        }
    }
}
impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}\n]",
            self.params
                .iter()
                .map(|f| { format!("\n  {}: {}", f.1, f.0) })
                .collect::<String>()
        )?;
        write!(f, " -> {}", self.return_type)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number(Number),
    String(TypeString),
    Boolean(Boolean),
    Void(Void),
    Function(Box<Function>),
}
impl Type {
    pub fn void() -> Self {
        Self::Void(Void {})
    }
    pub fn function(fn_decl: FunctionDeclaration) -> Self {
        Self::Function(Box::new(Function::from(&fn_decl)))
    }
    pub fn number() -> Self {
        Self::Number(Number {})
    }
    pub fn string() -> Self {
        Self::String(TypeString {})
    }
    pub fn boolean() -> Self {
        Self::Boolean(Boolean {})
    }

    pub fn is_void(&self) -> bool {
        matches!(self, Self::Void(_))
    }
    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function(_))
    }
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Number(_) => write!(f, "number"),
            Type::String(_) => write!(f, "string"),
            Type::Boolean(_) => write!(f, "bool"),
            Type::Void(_) => write!(f, "void"),
            Type::Function(_) => write!(f, "function"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeString {}

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {}

#[derive(Debug, Clone, PartialEq)]
pub struct Void {}
impl From<Literal> for Type {
    fn from(value: Literal) -> Self {
        match value.inner {
            lit_impl::Inner::Boolean(_) => Type::Boolean(Boolean {}),
            lit_impl::Inner::Number(_) => Type::Number(Number {}),
            lit_impl::Inner::String(_) => Type::String(TypeString {}),
        }
    }
}
impl From<Ident> for Type {
    fn from(value: Ident) -> Self {
        (&value).into()
    }
}

impl From<&Ident> for Type {
    fn from(value: &Ident) -> Self {
        let contents = value.name.as_ref();
        match contents {
            "number" => Type::Number(Number {}),
            "string" => Type::String(TypeString {}),
            "bool" => Type::Boolean(Boolean {}),
            "void" => Type::Void(Void {}),
            tmp => panic!("unexpected {tmp:?}"),
        }
    }
}

impl From<ReturnType> for Type {
    fn from(value: ReturnType) -> Self {
        value.ty.into()
    }
}
impl From<Expression> for Type {
    fn from(value: Expression) -> Self {
        match value {
            Expression::Lit(lit) => lit.into(),
            // this will work for now
            Expression::Binary(binary) => binary.into(),
            Expression::Call(_) => todo!(),
            Expression::Ident(_) => todo!(),
            Expression::Grouping(_) => todo!(),
            Expression::If(_) => todo!(),
            Expression::Block(_) => todo!(),
        }
    }
}
impl TypeCheck<(), ()> for Function {
    fn type_check(&mut self, other: (), _: ()) -> Result<(), Box<dyn Error>> {
        let return_type = &self.return_type;

        {
            let mut iter = self.fn_ast.body.0.iter().peekable();
            loop {
                let node = iter.next();

                let node_type: Type = match node {
                    Some(node) => match node {
                        Node::Statement(stmt) => stmt.clone().into(),
                        Node::Declaration(_) => todo!(),
                    },
                    None => break,
                };
                if iter.peek().is_none() && node_type != *return_type {
                    let err = format!(
                        "{} {} {expected} {} {actual}",
                        "Expected".bold().red(),
                        "return type of".bold(),
                        " but got".bold(),
                        expected = return_type.to_string().yellow(),
                        actual = node_type.to_string().yellow(),
                    );
                    println!("{}", err);
                    panic!();
                }
            }
        }
        Ok(())
    }
}
