use something_dev_tools::item_name;

use super::prelude::*;
use crate::prelude::*;
use crate::tokenizer::Parse;
mod function;
pub use self::function::*;
pub use self::var::*;

#[derive(Clone, ParseTokensDisplay)]
pub enum Declaration {
    Var(VariableDeclaration),

    Function(FunctionDeclaration),
}

impl Parse for Declaration {
    fn parse(parser: &mut crate::parser::Parser) -> crate::prelude::ParseResult<Self>
    where
        Self: Sized,
    {
        match Parse::parse(parser) {
            Ok(tmp) => return Ok(Self::Var(tmp)),
            Err(err) => return Err(err),
            Recoverable => {}
        };
        let tmp = parser.step(|parser| Parse::parse(parser));
        match tmp {
            Ok(tmp) => Ok(Self::Function(tmp)),
            Err(err) => Err(err),
            Recoverable => Recoverable,
        }
    }
}
impl std::fmt::Debug for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(arg0) => write!(f, "{:#?}", arg0),
            Self::Var(arg0) => write!(f, "{:#?}", arg0),
        }
    }
}
item_name!(Declaration, "declaration");
pub mod var;
