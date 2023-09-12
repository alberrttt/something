use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use something_ast::ast::{Ast, TopLevelNode};
use something_ast::prelude::*;
use something_ast::tokenizer::{Parse, ParsingDisplay, TokenStream};
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
            let mut tokens = something_ast::Parser::new("", file.as_str());
            let ast = match Ast::parse(&mut tokens) {
                Ok(ok) => ok,
                Err(err) => {
                    devprintln!("{err}");
                    panic!()
                }
                Recoverable => todo!(),
            };
            for node in ast.nodes {
                let TopLevelNode::FunctionDeclaration(fnd) = node;
                for node in fnd.body.iter() {
                    devprintln!("{}", ParsingDisplay::display(node));
                    // devprintln!("{:?}", node)
                }
            }
        }
    }
}
