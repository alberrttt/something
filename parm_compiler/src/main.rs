use std::{
    cell::RefCell,
    env,
    fs::{self},
    path::{Path, PathBuf},
    rc::Rc,
};

use parm_ast::{prelude::*, source_file::PreparsedSourceFile};
use parm_compiler::Config;
use parm_typechecker::{Scope, TypeChecker};

fn main() {
    let parm_toml = Path::new("./example/parm.toml");

    let file = fs::read_to_string(parm_toml).unwrap();
    let config: Config = toml::from_str(file.as_ref()).unwrap();
    let entry = PathBuf::from(config.package.bin.entry);
    let entry = PathBuf::from("./example")
        .join(entry)
        .canonicalize()
        .unwrap();

    let entry_src = fs::read_to_string(entry.clone()).unwrap();

    let preparsed_src_file = PreparsedSourceFile::new(entry, &entry_src);
    let (src_file, errors) = preparsed_src_file.parse();
    if env::var("TOKENS").is_ok() {
        for token in &src_file.preparsed.parser.tokens {
            println!("{:?}", token);
        }
    }
    for error in errors {
        eprintln!("{}", error);
    }
    if env::var("AST").is_ok() {
        for node in &src_file.ast {
            println!("{}", node.tree());
        }
    }
    let src_file = Rc::new(src_file);
    let mut typechecker = TypeChecker {
        source_file: src_file,
        scope: RefCell::new(Scope::default()),
    };
    let typechecked = typechecker.typecheck();
    // let mut lowering_ctx = LoweringCtx::new(&typechecked.typechecker);
    // dbg!(ast);
}
