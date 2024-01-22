use std::{
    cell::RefCell,
    env, fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use parm_compiler::{
    ast::{source_file::PreparsedSourceFile, tree_display::TreeDisplay},
    ir::{
        ir_scope::{IRScope, ScopeDeclaration},
        LoweringCtx,
    },
    opts::*,
    typechecker::{symbol::SymbolDeclaration, Scope, TypeChecker},
};

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
    let scope = unsafe { &*typechecker.scope.as_ptr() };
    for (idx, symbol) in scope.variables.iter() {
        let symbol = unsafe { &*symbol.as_ptr() };
        let Some(SymbolDeclaration::Function(function)) = &symbol.declaration else {
            continue;
        };
        let scope = unsafe { &*function.scope.as_ref().unwrap().as_ptr() };
        let mut ir_scope = IRScope {
            scope,
            declaration: ScopeDeclaration::FunctionDeclaration(function),
            children: vec![],
            prologue: vec![],
            epilogue: vec![],
            variables: Default::default(),
        };
        let ir = lowering.lower_fn(function.declaration, &mut ir_scope);
        println!("{:?}", ir);
    }
}
