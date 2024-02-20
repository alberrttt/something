use std::{
    env, fs,
    path::{Path, PathBuf},
};

use opts::Config;
use parm_ast::{
    error::ParseError, parser::nodes::item::Item, source_file::PreparsedSourceFile, traits::Node,
    tree_display::TreeDisplay,
};
use parm_hlir::{item::function::Function, traits::Check, ty};
mod opts;
fn main() {
    let parm_toml = Path::new("./example/parm.toml");

    let file = fs::read_to_string(parm_toml).unwrap();
    let config: Config = toml::from_str(file.as_ref()).unwrap();
    let entry = PathBuf::from(config.package.bin.entry);
    let entry = PathBuf::from("./example")
        .join(entry)
        .canonicalize()
        .unwrap();
    let src_str = fs::read_to_string(&entry).unwrap();
    let mut preparsed_file = PreparsedSourceFile::new(entry, &src_str);
    if env::var("TOKENS").is_ok() {
        println!("{:#?}", preparsed_file.lexer.tokens);
    }
    let mut parsed_file = preparsed_file.parse();
    if env::var("AST").is_ok() {
        for item in &parsed_file.ast {
            println!("{}", item.tree());
        }
    }
    for error in &parsed_file.errors {
        println!("{}", error);
    }
    let mut typechecker = parm_hlir::typechecker::Typechecker::new(&parsed_file);

    for item in &parsed_file.ast {
        let item = item.check(&mut typechecker);
        match item {
            Ok(item) => {
                println!("{:#?}", item);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    for errs in &typechecker.errs {
        println!("{}", errs);
    }
}
