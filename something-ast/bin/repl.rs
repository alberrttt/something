use something_ast::{
    ast::prelude::*,
    tokenizer::{Parse, ParsingDisplay},
};

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
    fn parse(parser: &mut something_ast::Parser) -> ParseResult<Self> {
        match parser.step(|parser| match parser.parse() {
            Ok(ok) => Ok(Repl::Expr(ok)),
            Err(err) => Err(err),
        }) {
            Ok(ok) => return Ok(ok),
            Recoverable => {}
            Err(err) => return Err(err),
        };
        devprintln!("parsing fn now");
        match parser.step(|parser| match FunctionDeclaration::parse(parser) {
            Ok(ok) => Ok(Repl::Fn(ok)),
            Err(err) => Err(err),
        }) {
            Ok(ok) => return Ok(ok),
            Err(err) => return Err(err),
        };
        devprintln!("parsing node now");
        match parser.step(|parser| match parser.parse() {
            Ok(ok) => Ok(Repl::Node(ok)),
            Err(err) => Err(err),
        }) {
            Ok(ok) => return Ok(ok),
            Err(err) => return Err(err),
        };
        panic!()
    }
}
impl Parse for Box<Repl> {
    fn parse(parser: &mut something_ast::parser::Parser) -> ParseResult<Self> {
        Ok(Box::new(Repl::parse(parser)?))
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
            Repl::Fn(_func) => std::fmt::Debug::fmt(&_func, f),
            Repl::Node(node) => node.fmt(f),
        }
    }
}
pub fn repl() {
    devprintln!(
        "{}\nRunning REPL.\nType `quit` to quit\n",
        format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).bold()
    );

    loop {
        print!("\n>> ");
        io::stdout().flush().unwrap(); // Flush stdout to ensure prompt is displayed

        let mut parser = String::new();
        io::stdin().read_line(&mut parser).unwrap(); // Read user parser

        let source = parser.trim(); // Remove trailing newline
        if source == "quit" {
            break; // Exit the loop if the user enters "quit"
        }
        let mut parser: something_ast::Parser<'_> = something_ast::Parser::new("file_name", source);
        let ast = Repl::parse(&mut parser);
        if let Ok(ast) = ast {
            devprintln!("{:?}", ast);
            if !parser.at_end() {
                print!("{}", "Error: ".bold().red());
                devprintln!("\nTokens left over: {:#?}\n", &parser.0[parser.1..]);
            }
        } else {
            devprintln!("{}", ast.err().unwrap());
            continue;
        }
    }
}
