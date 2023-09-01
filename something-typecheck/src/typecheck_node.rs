use std::{backtrace::Backtrace, cell::RefCell, rc::Rc};

use crate::{
    error::{TypeError, TypeErrorKind, TypeMismatch},
    scopes::CheckType,
    symbol::{Symbol, Type},
    type_check::TypeCheck,
    type_infer::InferLiteralType,
};
use something_ast::{
    ast::{expression, prelude::Declaration, Node},
    tokenizer::traits::AppendTokens,
};
use something_common::Result::{self, *};

impl TypeCheck for Node {
    type Output = Option<TypeError>;
    type With = Rc<RefCell<crate::scopes::Scope>>;
    type Against = Option<crate::symbol::Type>;

    fn type_check(&self, scope: Self::With, against: Self::Against) -> Self::Output {
        match self {
            Node::Declaration(decl) => {
                match decl {
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
                                        scope.borrow_mut().symbols.push(Rc::new(Symbol {
                                            name: var.name.to_string(),
                                            symbol_type: ty.clone(),
                                        }));
                                        return Some(TypeError {
                                        surrounding: Some(something_ast::tokenizer::traits::ToTokens::to_tokens(var)),
                                        kind: TypeErrorKind::Mismatch(
TypeMismatch::ExpressionTypeMismatch(
                                                (var.expression.clone(), expr),
                                                ty,
                                            ),
                                        ),
                                        backtrace: Some(Backtrace::capture()),
                                    });
                                    }
                                }
                                Err(mut err) => {
                                    var.clone().append_tokens(err.surrounding.as_mut().unwrap());
                                    return Some(err);
                                }
                                _ => todo!(),
                            }
                            return None;
                        }

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
                        None
                    }
                    Declaration::Function(_) => todo!(),
                }
            }
            Node::Statement(stmt) => match stmt {
                something_ast::ast::statement::Statement::Expression(expression) => todo!(),
                something_ast::ast::statement::Statement::Return(_) => todo!(),
            },
        }
    }
}
