use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use something_ast::ast::Ast;
use something_ast::prelude::*;
use something_ast::tokenizer::{Parse, TokenStream};
use something_common::Result::Recoverable;
mod repl;
#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Repl,
    Run { file: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Repl => repl::repl(),
        Commands::Run { file } => {
            let file = fs::read_to_string(file).unwrap();
            let mut tokens = TokenStream::from(file.as_str());
            let ast = match Ast::parse(&mut tokens) {
                Ok(ok) => ok,
                Err(err) => {
                    println!("{err}");
                    panic!()
                }
                Recoverable => todo!(),
            };
            println!("{:?}", &ast);
        }
    }
}
