use Macros::Tkn;

use crate::tokenizer::list::List;

use super::prelude::*;

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct Attribute {
    pub dollar: Tkn![$],
    pub brackets: Bracket<List<Ident>>,
    pub colon: Tkn![:],
}

mod __attribute {
    use super::Attribute;
    use crate::prelude::*;
    use crate::tokenizer::prelude::*;
    use colored::Colorize;
    use std::fmt::{Display, Formatter};
    impl Parse for Attribute {
        fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
            let tmp = parser.step(|parser| Parse::parse(parser));
            match tmp {
                Ok(tmp) => Ok(Self {
                    dollar: tmp,
                    brackets: Parse::parse(parser)?,
                    colon: Parse::parse(parser)?,
                }),
                Err(err) => Err(err),
            }
        }
    }
    impl AppendTokens for Attribute {
        fn append_tokens(&self, tokens: &mut TokenStream) {
            self.dollar.clone().append_tokens(tokens);
            self.brackets.clone().append_tokens(tokens);
            self.colon.clone().append_tokens(tokens);
        }
    }
    impl Parse for Box<Attribute> {
        fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
            Ok(Box::new(Attribute::parse(parser)?))
        }
    }
}
pub use __attribute::*;
