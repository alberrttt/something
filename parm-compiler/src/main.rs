use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
};

use parm_ast::{prelude::*, source_file::PreparsedSourceFile};
use parm_compiler::Config;

fn main() {
    let parm_toml = Path::new("./parm.toml");

    // let file = fs::read_to_string(parm_toml).unwrap();
    // let config: Config = toml::from_str(file.as_ref()).unwrap();
    // let entry = PathBuf::from(config.package.bin.entry);
    // let entry = entry.canonicalize().unwrap();

    // let entry_src = fs::read_to_string(entry.clone()).unwrap();
    let entry_src = String::from(
        r#"
    

fn main() {
    foo(4);  // 8 
} -> void
"#,
    );
    let preparsed_src_file = PreparsedSourceFile::new(PathBuf::from("test"), &entry_src);
    let (ast, errors) = preparsed_src_file.parse();

    for error in errors {
        eprintln!("{}", error);
    }
    if env::var("AST").is_ok() {
        println!("{:#?}", ast);
    }
    let mut typechecked = parm_typechecker::TypeCheckedSourceFile::new(ast);
    typechecked.typecheck();
    // dbg!(ast);
}
