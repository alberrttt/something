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
    fn parse(input: &mut Tokens) -> ParseResult<Self> {
        match input.step(|input| match input.parse() {
            Ok(ok) => Ok(Repl::Expr(ok)),
            Recoverable => Recoverable,
            Err(err) => Err(err),
        }) {
            Ok(ok) => return Ok(ok),
            Recoverable => {}
            Err(err) => return Err(err),
        };

        match input.step(|input| match input.parse() {
            Ok(ok) => Ok(Repl::Fn(ok)),
            Recoverable => Recoverable,
            Err(err) => Err(err),
        }) {
            Ok(ok) => return Ok(ok),
            Recoverable => {}
            Err(err) => return Err(err),
        };
        match input.step(|input| match input.parse() {
            Ok(ok) => Ok(Repl::Node(ok)),
            Recoverable => Recoverable,
            Err(err) => Err(err),
        }) {
            Ok(ok) => return Ok(ok),
            Recoverable => {}
            Err(err) => return Err(err),
        };
        panic!()
    }
}
impl Parse for Box<Repl> {
    fn parse(input: &mut Tokens) -> ParseResult<Self> {
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
        print!("\n>> ");
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
            println!("{:?}", ast);
            if !tokens.at_end() {
                print!("{}", "Error: ".bold().red());
                println!("\nTokens left over: {:#?}\n", &tokens.0[tokens.1..]);
            }
        } else {
            println!("{}", ast.err().unwrap());
            continue;
        }
    }
}
