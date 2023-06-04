use something_ast::{ast, Ast};
use something_typechecker::context::file::FileContext;

const SRC: &str = r#"

    fn x(number y) {
        return y + 1;
    } -> number

    fn main() {
        let abc = 1;
        let dca: string = abc;
        x(abc);
    } -> void
"#;

pub fn main() {
    let (ast, tokens): (Ast, _) = ast!(SRC);
    let ctx = FileContext::default();
    let file_ctx: FileContext = ctx.typecheck(ast).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
    println!("{:#?}", file_ctx);
}
