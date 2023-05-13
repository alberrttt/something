use clap::{Parser, Subcommand};
use something_ast::{declaration::FunctionDeclaration, expression::Expression, Ast};
use something_frontend_tokenizer::{Parse, ParsingDisplay, Tokens};
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
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Repl => repl::repl(),
    }
}
