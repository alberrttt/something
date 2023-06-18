use crate::ast;
use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;  
#[test]
fn test() {
    let (var, _): (VariableDeclaration, _) = something_ast::ast!("let var = 1;");
    let mut tokens = TokenStream::new();
    var.append_tokens(&mut tokens);
    dbg!(tokens);
}
