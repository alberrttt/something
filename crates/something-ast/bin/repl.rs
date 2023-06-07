use something_ast::{
    ast::prelude::*,
    tokenizer::{Parse, ParsingDisplay},
};
use something_dev_tools::*;
use std::{
    fmt::{Debug, Display},
    io::{self, Write},
};
pub enum Repl {
    Expr(Expression),
    Fn(FunctionDeclaration),
    Node(Node),
}
use colored::Colorize;
use something_ast::tokenizer::prelude::*;
impl Parse for Repl {
    fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
        let mut err = String::from("Expected ").yellow().to_string();
        match input.step(|input| Parse::parse(input)) {
            Ok(variant) => return Ok(Repl::Expr(variant)),
            Err(x) => {
                err.push_str(concat!(stringify!(Expr), ", "));
            }
        }
        match input.step(|input| Parse::parse(input)) {
            Ok(variant) => return Ok(Repl::Fn(variant)),
            Err(x) => {
                err.push_str(concat!(stringify!(Fn), ", "));
            }
        }
        match input.step(|input| Parse::parse(input)) {
            Ok(variant) => return Ok(Repl::Node(variant)),
            Err(x) => {
                err.push_str(concat!("or ", stringify!(Node)));
            }
        }
        err.push_str(format!("\n{} {}", "But got:".red(), input.peek().unwrap()).as_str());
        Err(ParseError::Generic(err))
    }
}
impl Parse for Box<Repl> {
    fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
        Ok(Box::new(Repl::parse(input)?))
    }
}
impl ParsingDisplay for Repl {
    fn display(&self) -> String
    where
        Self: Sized,
    {
        match self {
            Repl::Expr(expr) => expr.display(),
            Repl::Fn(func) => func.display(),
            Repl::Node(node) => node.display(),
        }
    }
    fn placeholder() -> String
    where
        Self: Sized,
    {
        format!("<{}>", "Repl")
    }
}

impl Debug for Repl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Repl::Expr(expr) => expr.fmt(f),
            Repl::Fn(_func) => std::fmt::Debug::fmt(&self, f),
            Repl::Node(node) => node.fmt(f),
        }
    }
}
pub fn repl() {
    println!(
        "{}\nRunning REPL.\nType `quit` to quit\n",
        format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).bold()
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
