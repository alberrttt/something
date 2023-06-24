use something_ast::ast::Ast;
use something_typechecker::context::file::FileContext;

const SRC: &str = r#"

    fn x(number y) {
        return y + 1;
    } -> number

    fn main() {
        let abc = 1;
        let dca = ;
        x(abc);
    } -> void
"#;
mod hack {
    macro_rules! devprintln {
        ($($arg:tt)*) => {
            if cfg!(debug_assertions) {
                print!(concat!("[",file!(), ":", line!(), "]: "));
                println!($($arg)*);

            }
        }
    }
    pub(super) use devprintln;
}
pub fn main() {
    let (ast, _tokens): (Ast, _) = something_ast::ast!(SRC);
    let ctx = FileContext::default();
    use hack::devprintln;
    let file_ctx: FileContext = ctx.typecheck(ast).unwrap_or_else(|err| {
        devprintln!("Error: {}", err);
        std::process::exit(1);
    });
    devprintln!("{:#?}", file_ctx);
}
