#![feature(associated_type_defaults)]
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use scopes::Scope;
use something_ast::{ast::prelude::*, prelude::devprintln};
use symbol::{FnSig, Symbol, SymbolTable, Type};
use type_infer::{InferLiteralType, InferType};
mod error;
mod scopes;
mod symbol;
mod type_infer;
#[derive(Clone)]
pub struct Module<'a> {
    pub declarations: &'a [Declaration],
    pub module_symbols: Vec<Symbol>,
    pub fn_scopes: Vec<Scope>,
}
#[allow(non_camel_case_types)]
struct __debug_hack(String);
impl std::fmt::Debug for __debug_hack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<'a> std::fmt::Debug for Module<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field(
                "declarations",
                &self
                    .declarations
                    .iter()
                    .map(|f| {
                        __debug_hack(match f {
                            Declaration::Var(var) => format!("var<{}>", var.name.to_string()),
                            Declaration::Function(f) => format!("fn<{}>", f.name.to_string()),
                        })
                    })
                    .collect::<Vec<_>>(),
            )
            .field("module_symbols", &self.module_symbols)
            .field("fn_scopes", &self.fn_scopes)
            .finish()
    }
}
impl<'a> Module<'a> {
    pub fn new(declarations: &'a [Declaration]) -> Self {
        Self {
            declarations,
            module_symbols: Vec::new(),
            fn_scopes: Vec::new(),
        }
    }

    pub fn populate_symbol_table(&mut self) {
        for decl in self.declarations {
            match decl {
                Declaration::Function(function) => {
                    self.add_function_to_symbol_table(function.clone());
                }
                Declaration::Var(variable) => {
                    self.add_variable_to_symbol_table(variable);
                }
            }
        }
    }

    fn add_function_to_symbol_table(&mut self, function: FunctionDeclaration) {
        let params = function
            .params
            .iter()
            .map(|((ty, ident), _)| {
                Rc::new(Symbol {
                    symbol_type: ty.infer_literal_type().unwrap(),
                    name: ident.to_string(),
                })
            })
            .collect();

        let fn_type = FnSig {
            params,
            return_type: function.return_type.ty.infer_literal_type().unwrap(),
        };

        self.module_symbols.push(Symbol {
            symbol_type: Type::Function(Box::new(fn_type.clone())),
            name: function.name.to_string(),
        });
        let (scope, errs) = Scope::create_scope_from_function(function, fn_type);
        errs.iter().for_each(|f| {
            println!("{}", f);
        });
        self.fn_scopes.push(scope);
    }
    #[allow(unreachable_code)]
    fn add_variable_to_symbol_table(&mut self, variable: &VariableDeclaration) {
        // we need to add literal type inference or make use explicit type annotation
        let symbol_type = todo!();
        self.module_symbols.push(Symbol {
            name: variable.name.to_string(),
            symbol_type,
        });
    }
}
#[test]
fn test() {
    let (decls, _): (List<Declaration>, _) = something_ast::ast!(
        "
    fn x(number x, number y) { 
        let a: bool = 1;
        let b = true;
        let z: number = y + x + a + b;
    } -> void

    "
    );
    let mut module = Module::new(&decls);
    module.populate_symbol_table();
    devprintln!("{:#?}", module);
}
mod type_check;

trait FindSymbolHack {
    fn find_symbol(&self, name: &str) -> Option<&Symbol>;
}
impl FindSymbolHack for Vec<Symbol> {
    fn find_symbol(&self, name: &str) -> Option<&Symbol> {
        self.iter().find(|s| s.name == name)
    }
}
