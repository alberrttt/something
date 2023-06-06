use something_ast::prelude::VariableDeclaration;
use something_frontend_tokenizer::{traits::AppendTokens, Tokens};

#[test]
fn test() {
    let (var, _): (VariableDeclaration, _) = something_ast::ast!("let var = 1;");
    let mut tokens = Tokens::new();
    var.append_tokens(&mut tokens);
    dbg!(tokens);
}
