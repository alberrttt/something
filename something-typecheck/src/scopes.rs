use std::{
    backtrace::{self, Backtrace},
    cell::{RefCell, UnsafeCell},
    collections::HashSet,
    fmt::Debug,
    marker::PhantomData,
    rc::Rc,
};

use something_ast::{
    ast::{
        prelude::{Declaration, Expression, FunctionDeclaration},
        Node,
    },
    tokenizer::{
        prelude::Ident,
        traits::{AppendTokens, ToTokens},
        TokenStream,
    },
};

use crate::{
    error::TypeError,
    symbol::{FnSig, Symbol, Type},
    type_check::TypeCheck,
    type_infer::{InferLiteralType, InferType},
    Module,
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
pub trait CheckType<'a> {
    type With = ();
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
                let right: Type = binary.right.resolve_type(with.clone(), against.clone())?;
                if let Some(against) = &against {
                    if &left != against && &right != against {}
                }
                if left == right {
                    Ok(left)
                } else {
                    Err(TypeError::IncompatibleBinaryOperation(
                        (*binary.left.to_owned(), left),
                        (*binary.right.to_owned(), right),
                        binary.operator.clone(),
                        TokenStream::new(),
                    ))
                }
            }
            Expression::Call(_) => todo!(),
            Expression::Ident(ident) => {
                if let Some(scope) = with {
                    let scope = scope.borrow();
                    match scope
                        .resolve_symbol(ident.name.as_str())
                        .map(|symbol| symbol.symbol_type.clone())
                        .ok_or(TypeError::UndefinedIdentifier(
                            ident.clone(),
                            TokenStream::new(),
                        )) {
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
    pub fn create_scope_from_function(
        module: &mut Module,
        function: FunctionDeclaration,
        fn_sig: FnSig,
    ) -> (Self, Vec<TypeError>) {
        let mut symbols = fn_sig.params.to_vec();
        // todo: there might be functions beneath this one, which we are unaware of
        // solution: first iterate and get the fn sig for every function, withotu checking its body
        // then we can add it to the module symbols,
        // and when we finaly get to it, we can check the body.

        for symbol in &module.module_symbols {
            dbg!(symbol);
            symbols.push(symbol.clone());
        }
        let scope = Rc::new(RefCell::new(Self {
            symbols,
            parent: None,
        }));
        let mut errors = vec![];
        for stmt in function.body.iter() {
            if let Some(err) = stmt.type_check(scope.clone(), None) {
                errors.push(err);
            }
        }
        (Rc::into_inner(scope).unwrap().into_inner(), errors)
    }
}
