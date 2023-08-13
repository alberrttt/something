use std::collections::HashSet;

use something_ast::{ast::prelude::*, prelude::devprintln};
use symbol::{Symbol, SymbolTable, Type};

mod symbol;

#[derive(Clone)]
pub struct Module<'a> {
    pub declarations: &'a [Declaration],
    pub module_symbols: HashSet<Symbol>,
}
impl<'a> std::fmt::Debug for Module<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field(
                "declarations",
                &self
                    .declarations
                    .iter()
                    .map(|f| match f {
                        Declaration::Var(var) => format!("var<{}>", var.name.to_string()),
                        Declaration::Function(f) => format!("fn<{}>", f.name.to_string()),
                    })
                    .collect::<Vec<_>>(),
            )
            .field("module_symbols", &self.module_symbols)
            .finish()
    }
}
impl<'a> Module<'a> {
    pub fn new(declarations: &'a [Declaration]) -> Self {
        Self {
            declarations,
            module_symbols: HashSet::new(),
        }
    }

    pub fn populate_symbol_table(&mut self) {
        for decl in self.declarations {
            match decl {
                Declaration::Function(function) => {
                    self.add_function_to_symbol_table(function);
                }
                Declaration::Var(variable) => {
                    self.add_variable_to_symbol_table(variable);
                }
            }
        }
    }

    fn add_function_to_symbol_table(&mut self, function: &FunctionDeclaration) {
        self.module_symbols.insert(Symbol {
            symbol_type: Type::Function,
            name: function.name.to_string(),
        });
    }
    #[allow(unreachable_code)]
    fn add_variable_to_symbol_table(&mut self, variable: &VariableDeclaration) {
        // we need to add literal type inference or make use explicit type annotation
        let symbol_type = todo!();
        self.module_symbols.insert(Symbol {
            name: variable.name.to_string(),
            symbol_type,
        });
    }
}
#[test]
fn test() {
    let (decls, _): (List<Declaration>, _) = something_ast::ast!(
        "
    fn x(float x, int y) { 
        let z: float = y + x;
    } -> void
    fn a(float b, int c) { 
        let d: float = c + b;
    } -> void
    "
    );
    let mut module = Module::new(&decls);
    module.populate_symbol_table();
    devprintln!("{:#?}", module);
}
