use std::{collections::HashSet, rc::Rc};

use something_ast::{
    ast::{
        prelude::{Declaration, Expression, FunctionDeclaration},
        Node,
    },
    tokenizer::prelude::Ident,
};

use crate::{
    error::TypeError,
    symbol::{FnSig, Symbol, Type},
    type_infer::{InferLiteralType, InferType},
};
use something_common::{
    devprintln,
    Result::{self, *},
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    pub symbols: Vec<Rc<Symbol>>,
    pub parent: Option<Rc<Scope>>,
}
trait ResolveType<'a> {
    type With = &'a ();
    type Output = Result<Type, TypeError>;
    fn resolve_type(&self, with: Self::With) -> Self::Output;
}

impl<'a> ResolveType<'a> for Expression {
    type With = Option<&'a mut Scope>;

    fn resolve_type(&self, mut with: Self::With) -> Self::Output {
        let tmp = match self {
            Expression::Lit(lit) => lit.infer_literal_type(),
            Expression::Binary(binary) => todo!(),
            Expression::Call(_) => todo!(),
            Expression::Ident(ident) => {
                if let Some(scope) = &mut with {
                    match scope
                        .resolve_symbol(ident.name.as_str())
                        .map(|symbol| symbol.symbol_type.clone())
                        .ok_or(TypeError::Generic("Symbol not found"))
                    {
                        std::result::Result::Ok(ok) => Ok(ok),
                        std::result::Result::Err(err) => Err(err),
                    }
                } else {
                    panic!()
                }
            }
            Expression::Grouping(_) => todo!(),
            Expression::If(_) => todo!(),
            Expression::Block(_) => todo!(),
        };
        tmp
    }
}
impl Scope {
    pub fn resolve_symbol(&self, name: &str) -> Option<Rc<Symbol>> {
        self.symbols
            .iter()
            .find(|symbol| symbol.name == name)
            .cloned()
    }
    pub fn create_scope_from_function(function: FunctionDeclaration, fn_sig: FnSig) -> Self {
        let mut symbols: Vec<_> = fn_sig.params.to_vec();
        let mut scope = Self {
            symbols,
            parent: None,
        };
        for stmt in function.body.iter() {
            match stmt {
                Node::Declaration(decl) => match decl {
                    Declaration::Var(var) => {
                        let (expr) = var.expression.clone().resolve_type(Some(&mut scope));
                        if let Ok(ty) = var.infer_literal_type() {
                            match expr {
                                Ok(expr) => {
                                    if expr == ty {
                                        scope.symbols.push(Rc::new(Symbol {
                                            name: var.name.to_string(),
                                            symbol_type: { ty },
                                        }));
                                    } else {
                                        devprintln!(
                                            "Type mismatch: expected {}, found {}",
                                            ty,
                                            expr
                                        );
                                    }
                                }
                                Err(err) => {
                                    devprintln!("{}", err);
                                }
                                _ => todo!(),
                            }
                        } else {
                            scope.symbols.push(Rc::new(Symbol {
                                name: var.name.to_string(),
                                symbol_type: { expr.unwrap() },
                            }));
                        }
                    }
                    Declaration::Function(_) => todo!(),
                },
                Node::Statement(_) => todo!(),
            }
        }
        scope
    }
}
