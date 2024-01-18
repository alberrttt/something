use std::{
    cell::RefCell,
    env,
    fs::{self},
    path::{Path, PathBuf},
    rc::Rc,
};

use parm_ast::{prelude::*, source_file::PreparsedSourceFile};
use parm_compiler::Config;
use parm_ir::LoweringCtx;
use parm_typechecker::{symbol::SymbolDeclaration, Scope, TypeChecker};

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
        panic: RefCell::new(false),
    };
    typechecker.typecheck();
    if *typechecker.panic.borrow() {
        return;
    }
    let mut lowering = LoweringCtx::new(&typechecker);
    for (idx, symbol) in &typechecker.scope.borrow().variables {
        let symbol = symbol.borrow();
        let Some(SymbolDeclaration::Function(function)) = &symbol.declaration else {
            continue;
        };
        let ir = lowering.lower_fn(function.declaration, {
            let tmp = function.scope.as_ref();
            // lol
            unsafe { &*tmp.unwrap().as_ptr() }
        });
        println!("{:?}", ir);
    }
}
