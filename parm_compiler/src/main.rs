use std::{
    env, fs,
    path::{Path, PathBuf},
};

use opts::Config;
use parm_ast::{
    parser::nodes::item::Item, source_file::PreparsedSourceFile, traits::Node,
    tree_display::TreeDisplay,
};
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
    let mut file = PreparsedSourceFile::new(entry, &src_str);

    let items: Result<Vec<Item<'_>>, Box<parm_ast::prelude::ParseError<'_>>> =
        <Vec<Item> as Node<'_>>::parse(&mut file.parser.stream(&file));
    let items = items.unwrap();
    let mut main = None;
    for item in &items {
        if let Item::Function(funct) = &item {
            if funct.name.lexeme == "main" {
                main = Some(funct);
            }
        }
    }
    let main = main.unwrap();

    let mut typechecker = parm_hlir::typechecker::Typechecker::new(&file);
    let main = typechecker.check_fn(main);

    dbg!(main);
}
