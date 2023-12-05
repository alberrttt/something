use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use parm_ast::{prelude::*, source_file::PreparsedSourceFile};
use parm_compiler::Config;
fn main() {
    let parm_toml = Path::new("./parm.toml");

    let file = fs::read_to_string(parm_toml).unwrap();
    let config: Config = toml::from_str(file.as_ref()).unwrap();
    let entry = PathBuf::from(config.package.bin.entry);
    let entry = entry.canonicalize().unwrap();

    let entry_src = fs::read_to_string(entry.clone()).unwrap();

    let mut parser = Parser::new(entry_src.as_ref());
    let preparsed_src_file = PreparsedSourceFile::new(entry.clone(), &entry_src);
    let (ast, errors) = preparsed_src_file.parse();
    for error in errors {
        eprintln!("{}", error);
    }
    dbg!(ast);
}
