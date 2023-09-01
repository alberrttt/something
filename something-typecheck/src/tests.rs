use something_ast::{ast::prelude::Declaration, tokenizer::prelude::List};
use something_common::devprintln;

use crate::Module;

#[test]
fn test_two_fns() {
    let (decls, _): (List<Declaration>, _) = something_ast::ast!(
        "
   
    fn square(number i) {
        return i*i;
    } -> number
    fn x(number c) { 
        let a: bool = c/2;
        let z: number = square;
    } -> void
    "
    );
    let mut module = Module::new(&decls);
    module.populate_symbol_table();
    devprintln!("{:#?}", module);
}
