use something_ast::expression::Expression;
use something_frontend_tokenizer::Parse;

fn main() {
    let mut tokens = something_frontend_tokenizer::Tokenizer::new(include_str!("./cases/expr.txt"))
        .tokens()
        .unwrap();
    dbg!(tokens.peek());
    dbg!(Expression::parse(&mut tokens));
}
