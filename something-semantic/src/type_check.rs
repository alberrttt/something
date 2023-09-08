use crate::symbol::Type;
use std::{backtrace::Backtrace, cell::RefCell, rc::Rc};

pub trait TypeCheck {
    type Output;
    type With;
    type Against = Option<Type>;
    fn type_check(&self, with: Self::With, against: Self::Against) -> Self::Output;
}

use crate::{
    error::{TypeError, TypeErrorKind, TypeMismatch},
    scopes::CheckType,
    symbol::{FnSig, Symbol},
    type_infer::InferLiteralType,
};
use something_ast::{
    ast::{
        expression,
        prelude::{Declaration, Expression, VariableDeclaration},
        Node,
    },
    tokenizer::{
        traits::{AppendTokens, ToTokens},
        TokenStream,
    },
};
use something_common::Result::{self, *};

impl TypeCheck for Node {
    type Output = Option<TypeError>;
    type With = Rc<RefCell<crate::scopes::Scope>>;
    type Against = Option<Rc<FnSig>>;

    fn type_check(&self, scope: Self::With, against: Self::Against) -> Self::Output {
        match self {
            Node::Declaration(decl) => match decl {
                Declaration::Var(var) => var.type_check(scope, against),
                Declaration::Function(_) => todo!(),
            },
            Node::Statement(stmt) => match stmt {
                something_ast::ast::statement::Statement::Expression(expression) => {
                    let tmp: Result<Type, TypeError> = expression.0.resolve_type(Some(scope), None);
                    match tmp.err() {
                        Some(mut some) => {
                            stmt.append_tokens(some.surrounding.as_mut().unwrap());
                            Some(some)
                        }
                        None => None,
                    }
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
impl TypeCheck for VariableDeclaration {
    type Output = Option<TypeError>;
    type With = Rc<RefCell<crate::scopes::Scope>>;
    type Against = Option<Rc<FnSig>>;

    fn type_check(&self, scope: Self::With, against: Self::Against) -> Self::Output {
        let expr_result = self
            .expression
            .clone()
            .resolve_type(Some(scope.clone()), None);
        if let Ok(expr_type) = self.infer_literal_type() {
            match expr_result {
                Ok(expr) => {
                    if expr != expr_type {
                        return handle_type_mismatch(
                            expr,
                            expr_type,
                            self.name.to_string(),
                            self.expression.clone(),
                            self,
                            scope,
                        );
                    } else {
                        scope.borrow_mut().symbols.push(Rc::new(Symbol {
                            name: self.name.to_string(),
                            symbol_type: expr_type,
                        }));
                        return None;
                    }
                }
                Err(mut err) => {
                    self.append_tokens(err.surrounding.as_mut().unwrap());
                    scope.borrow_mut().symbols.push(Rc::new(Symbol {
                        name: self.name.to_string(),
                        symbol_type: expr_type.clone(),
                    }));
                    return Some(err);
                }
                _ => return todo!(),
            }
        }

        match expr_result {
            Ok(expr_type) => {
                let symbol = Rc::new(Symbol {
                    name: self.name.to_string(),
                    symbol_type: expr_type,
                });
                scope.borrow_mut().symbols.push(symbol);
                None
            }
            _ => {
                let tmp: Result<Type, TypeError> =
                    self.expression.resolve_type(Some(scope.clone()), None);
                match tmp {
                    Ok(ok) => {
                        let symbol = Rc::new(Symbol {
                            name: self.name.to_string(),
                            symbol_type: ok,
                        });
                        scope.borrow_mut().symbols.push(symbol);
                        None
                    }
                    Recoverable => todo!(),
                    Err(mut err) => {
                        self.append_tokens(err.surrounding.as_mut().unwrap());
                        Some(err)
                    }
                }
            }
        }
    }
}
fn handle_type_mismatch(
    expr: Type,
    expr_type: Type,
    name: String,
    expression: Expression,
    to_tokens: &dyn ToTokens,
    scope: Rc<RefCell<crate::scopes::Scope>>,
) -> Option<TypeError> {
    let symbol = Rc::new(Symbol {
        name: name.clone(),
        symbol_type: expr_type.clone(),
    });
    scope.borrow_mut().symbols.push(symbol);

    Some(TypeError {
        surrounding: Some(to_tokens.to_tokens()),
        kind: TypeErrorKind::Mismatch(TypeMismatch::ExpressionTypeMismatch(
            (expression.clone(), expr),
            expr_type,
        )),
        backtrace: Some(Backtrace::capture()),
    })
}
