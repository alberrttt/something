use std::{
    cell::{RefCell, UnsafeCell},
    env, fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use parm_compiler::{
    ast::{
        error::ParseError,
        parser::{nodes::item::Item, ParseStream},
        source_file::{PreparsedSourceFile, SourceFile},
        traits::Node,
        tree_display::TreeDisplay,
    },
    opts::*,
    typechecker::{scope::ScopeArena, ty, Typechecker},
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
    let (mut src_file, errors) = {
        let mut stream = ParseStream {
            tokens: &preparsed_src_file.parser.tokens,
            current: 0,
            src_file: &preparsed_src_file,
            panic: false,
            attributes: Default::default(),
            errors: Default::default(),
        };
        let (ast, errors) =
            <Vec<Item<'_>> as Node<'_, (Vec<Item<'_>>, Vec<ParseError<'_>>)>>::parse(&mut stream);
        (
            SourceFile {
                preparsed: &preparsed_src_file,
                ast,
            },
            errors,
        )
    };
    if env::var("TOKENS").is_ok() {
        for token in &src_file.preparsed.parser.tokens {
            println!("{:?}", token);
        }
    }
    for error in errors {
        eprintln!("{}", error);
    }

    let mut typechecker = Typechecker {
        source_file: &mut src_file,
        scopes: ScopeArena::new(),
        ty_arena: Default::default(),
    };

    let typechecker = UnsafeCell::new(typechecker);
    unsafe {
        (*typechecker.get()).check().unwrap();
        let tc = &(*typechecker.get());
        if env::var("AST").is_ok() {
            for node in &tc.source_file.ast {
                println!("{:#?}", node);
            }
        }
    }
}
