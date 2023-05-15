use something_ast::prelude::*;
use std::{
    fmt::{Debug, Display},
    io::{self, Write},
};
#[derive(ParseTokensDisplay, ParseTokens)]
pub enum Repl {
    Expr(Expression),
    Fn(FunctionDeclaration),
    Node(Node),
}

impl Debug for Repl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Repl::Expr(expr) => expr.fmt(f),
            Repl::Fn(func) => func.fmt(f),
            Repl::Node(node) => node.fmt(f),
        }
    }
}
use colored::Colorize;
pub fn repl() {
    println!(
        "{} {}\nRunning REPL.\nType `quit` to quit",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    loop {
        print!(">> ");
        io::stdout().flush().unwrap(); // Flush stdout to ensure prompt is displayed

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read user input

        let input = input.trim(); // Remove trailing newline

        if input == "quit" {
            break; // Exit the loop if the user enters "quit"
        }
        let mut tokens: Tokens = input.into();

        let ast = Repl::parse(&mut tokens);
        if let Ok(ast) = ast {
            println!("{}", ast.display());
            println!("{:?}", ast);
            if !tokens.at_end() {
                print!("{}", "Error: ".bold().red());
                println!("\nTokens left over: {:#?}\n", &tokens.0[tokens.1..]);
            }
        } else {
            print!("{}", "Error: ".bold().red());
            println!("{}", ast.err().unwrap());
            continue;
        }
    }
}
