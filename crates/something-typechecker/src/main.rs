use something_ast::{ast, Ast};
use something_typechecker::context::file::FileContext;

const SRC: &str = r#"

    fn x(number y) {
        return y + 1;
    } -> number

    fn main() {
        let abc = 1;
        x(abc);
    } -> void
"#;

pub fn main() {
    let (ast, tokens): (Ast, _) = ast!(SRC);
    let file_ctx: FileContext = ast.into();
    println!("{:#?}", file_ctx);
}
