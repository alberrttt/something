use prelude::PreparsedSourceFile;

pub mod error;
pub mod lexer;
pub mod parser;
pub mod prelude;
pub mod source_file;
pub mod tests;
pub mod traits;
pub mod tree_display;

#[test]
fn test() {
    let src = "fn main() {1()}";
    let mut parser = PreparsedSourceFile::new("test".into(), src);
    let file = parser.parse();
    dbg!(file);
}
