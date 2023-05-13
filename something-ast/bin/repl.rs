use something_ast::prelude::*;
use std::io::{self, Write};
pub fn repl() {
    println!(
        "version {} Running REPL.\nType `quit` to quit",
        env!("CARGO_PKG_VERSION")
    );
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Flush stdout to ensure prompt is displayed

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read user input

        let input = input.trim(); // Remove trailing newline

        if input == "quit" {
            break; // Exit the loop if the user enters "quit"
        }
        let mut tokens: Tokens = input.into();

        if let Ok(ast) = Expression::parse(&mut tokens) {
            println!("{}", ast.display());
        } else if let Ok(ast) = Node::parse(&mut tokens) {
            println!("{}", ast.display());
        } else if let Ok(ast) = FunctionDeclaration::parse(&mut tokens) {
            println!("{}", ast.display());
        } else {
            println!("Error: {:?}", tokens);
        }
    }
}
