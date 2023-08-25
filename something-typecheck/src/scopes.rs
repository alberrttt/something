use std::{
    cell::{RefCell, UnsafeCell},
    collections::HashSet,
    rc::Rc,
};

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
trait CheckType<'a> {
    type With = &'a ();
    type Against = ();
    type Output = Result<Type, TypeError>;
    fn resolve_type(&self, with: Self::With, against: Self::Against) -> Self::Output;
}

impl<'a> CheckType<'a> for Expression {
    type With = Option<Rc<RefCell<Scope>>>;
    type Against = Option<Type>;
    fn resolve_type(&self, with: Self::With, against: Self::Against) -> Self::Output {
        let tmp = match self {
            Expression::Lit(lit) => lit.infer_literal_type(),
            Expression::Binary(binary) => {
                // TODO: Probably check if the operator is valid for the types
                // Also, have a type error like: "Expected X type, but got a binary expression with Y and Z types"
                let left: Type = binary.left.resolve_type(with.clone(), against.clone())?;
                let right: Type = binary.right.resolve_type(with.clone(), against)?;
                if left == right {
                    Ok(left)
                } else {
                    Err(TypeError::Generic(format!(
                        "Type mismatch: expected {}, found {}",
                        left, right
                    )))
                }
            }
            Expression::Call(_) => todo!(),
            Expression::Ident(ident) => {
                if let Some(scope) = with {
                    let scope = scope.borrow();
                    match scope
                        .resolve_symbol(ident.name.as_str())
                        .map(|symbol| symbol.symbol_type.clone())
                        .ok_or(TypeError::Generic(format!(
                            "Symbol not found, got ident `{}`",
                            ident.name
                        ))) {
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
        let mut scope = Rc::new(RefCell::new(Self {
            symbols,
            parent: None,
        }));

        for stmt in function.body.iter() {
            match stmt {
                Node::Declaration(decl) => match decl {
                    Declaration::Var(var) => {
                        let expr: something_common::Result<Type, TypeError> = var
                            .expression
                            .clone()
                            .resolve_type(Some(scope.clone()), None);

                        if let Ok(ty) = var.infer_literal_type() {
                            match expr {
                                Ok(expr) => {
                                    if expr == ty {
                                        scope.borrow_mut().symbols.push(Rc::new(Symbol {
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
                            let symbol = Rc::new(Symbol {
                                name: var.name.to_string(),
                                symbol_type: match expr {
                                    Ok(ok) => ok,
                                    _ => var
                                        .expression
                                        .resolve_type(Some(scope.clone()), None)
                                        .unwrap(),
                                },
                            });
                            scope.borrow_mut().symbols.push(symbol);
                        }
                    }
                    Declaration::Function(_) => todo!(),
                },
                Node::Statement(_) => todo!(),
            }
        }
        Rc::into_inner(scope).unwrap().into_inner()
    }
}
