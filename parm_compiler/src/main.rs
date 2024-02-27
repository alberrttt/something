use std::{
    cell::RefCell,
    env, fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use opts::Config;
use parm_ast::{
    error::ParseError, parser::nodes::item::Item, source_file::PreparsedSourceFile, traits::Node,
    tree_display::TreeDisplay,
};
use parm_hlir::{
    item::function::Function,
    symbol::{InnerSymbol, Symbol, SymbolDeclaration},
    traits::Check,
    ty::{self, function_ty::FunctionTy, Type},
};
mod opts;
fn main() {
    let parm_toml = Path::new("./example/parm.toml");

    let file = fs::read_to_string(parm_toml).unwrap();
    let config: Config = toml::from_str(file.as_ref()).unwrap();
    let entry = PathBuf::from("./example/src/main.af");

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
    let x = typechecker.none_symbol.clone();
    typechecker.mut_current_scope().push_symbol({
        let symbol = Symbol::new(SymbolDeclaration::None, ty::Type::Any, "println");
        symbol.set_ty(Type::Function(Rc::new(FunctionTy {
            symbol: symbol.clone(),
            params: vec![x],
            return_ty: ty::Type::Unknown { err: false },
        })));
        symbol
    });
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
