use something_ast::ast::prelude::*;

use something_ast::tokenizer::prelude::*;
#[test]
fn test() {
    let (var, _): (VariableDeclaration, _) = something_ast::ast!("let var = 1;");
    let mut tokens = TokenStream::new();
    var.append_tokens(&mut tokens);
    dbg!(tokens);
}
