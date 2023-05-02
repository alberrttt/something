use something_ast::punctuated::Punctuated;
use something_frontend_tokenizer::{Tokenizer, lit::Literal, tokens};

#[test]
fn punctuated_terminating_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokens = Tokenizer::new(include_str!("../cases/punctuated_terminating.txt"))
        .tokens()
        .unwrap();
    dbg!(tokens.peek());

    dbg!(Punctuated::<Literal, tokens::Comma>::parse_terminated(
        &mut tokens
    )?);
    Ok(())
}
#[test]
fn punctuated_trailing_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokens = Tokenizer::new(include_str!("../cases/punctuated_trailing.txt"))
        .tokens()
        .unwrap();
    dbg!(tokens.peek());

    dbg!(Punctuated::<Literal, tokens::Comma>::parse_trailing(
        &mut tokens
    )?);
    Ok(())
}
#[test]
fn punctuated_no_trailing_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokens = Tokenizer::new(include_str!("../cases/punctuated_no_trailing.txt"))
        .tokens()
        .unwrap();
    dbg!(tokens.peek());

    dbg!(Punctuated::<Literal, tokens::Comma>::parse_without_trailing(&mut tokens)?);
    Ok(())
}
