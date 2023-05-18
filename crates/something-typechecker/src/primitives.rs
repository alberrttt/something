use core::panic;
use std::{error::Error, fmt::Display, rc::Rc};

use crate::{symbol::Symbol, traits::TypeCheck};
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
            Expression::Binary(binary) => match binary.operator {
                Operator::GreaterEqual => Type::Boolean(Boolean {}),
                Operator::LessEqual => Type::Boolean(Boolean {}),
                Operator::Greater => Type::Boolean(Boolean {}),
                Operator::Less => Type::Boolean(Boolean {}),
                Operator::EqualEqual => Type::Boolean(Boolean {}),

                _ => todo!(),
            },
            _ => panic!(),
        }
    }
}
impl TypeCheck<()> for Function {
    fn type_check(&mut self, other: ()) -> Result<(), Box<dyn Error>> {
        self.fn_ast
            .as_ref()
            .body
            .iter()
            .for_each(|stmt| match stmt {
                Node::Statement(stmt) => match stmt {
                    Statement::Expression(_, _) => todo!(),
                    Statement::Return(_, expr, _) => {
                        let expr_type: Type = expr.clone().into();
                        if expr_type != self.return_type {
                            panic!("expected {}, got {expr_type}", self.return_type)
                        }
                    }
                },
                Node::Declaration(_) => todo!(),
            });
        Ok(())
    }
}
