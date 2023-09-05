use std::{backtrace::Backtrace, cell::RefCell, rc::Rc};

use crate::{
    error::{TypeError, TypeErrorKind, TypeMismatch},
    scopes::CheckType,
    symbol::{FnSig, Symbol, Type},
    type_check::TypeCheck,
    type_infer::InferLiteralType,
};
use something_ast::{
    ast::{expression, prelude::Declaration, Node},
    tokenizer::{traits::AppendTokens, TokenStream},
};
use something_common::Result::{self, *};

impl TypeCheck for Node {
    type Output = Option<TypeError>;
    type With = Rc<RefCell<crate::scopes::Scope>>;
    type Against = Option<Rc<FnSig>>;

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
                something_ast::ast::statement::Statement::Expression(expression) => {
                    let tmp: Result<Type, TypeError> = expression.0.resolve_type(Some(scope), None);
                    tmp.err()
                }
                something_ast::ast::statement::Statement::Return(return_stmt) => {
                    let tmp: Type = match return_stmt.1.resolve_type(Some(scope), None) {
                        Ok(_) => return_stmt.1.infer_literal_type().unwrap_or(Type::Void),
                        Err(err) => {
                            return Some(err);
                        }
                        Recoverable => todo!(),
                    };
                    let mut surrounding = TokenStream::new();
                    return_stmt.clone().append_tokens(&mut surrounding);

                    match against {
                        Some(fn_sig) => {
                            if fn_sig.return_type.ne(&tmp) {
                                Some(TypeError::InvalidReturnType(
                                    fn_sig.as_ref().return_type.clone(),
                                    (tmp, {
                                        let mut tmp = TokenStream::new();
                                        return_stmt.1.clone().append_tokens(&mut tmp);
                                        tmp
                                    }),
                                    surrounding,
                                ))
                            } else {
                                None
                            }
                        }
                        None => todo!(),
                    }
                }
            },
        }
    }
}
