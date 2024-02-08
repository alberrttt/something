use std::rc::Rc;

use parm_common::Spanned;
use parm_dev_macros::Spanned;

use crate::ast::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement<'a> {
    pub let_tkn: Let<'a>,
    pub ident: Identifier<'a>,
    pub initializer: Option<Initializer<'a>>,
    pub semi: SemiColon<'a>,
}

impl Spanned for LetStatement<'_> {
    fn span(&self) -> parm_common::Span {
        self.let_tkn.span().join(self.semi.span())
    }
}

impl TreeDisplay for LetStatement<'_> {
    fn tree(&self) -> Tree {
        Tree::new("LetStatement")
            .child(self.let_tkn.tree().label("let_tkn"))
            .child(self.ident.tree().label("ident"))
            .child(self.initializer.tree().label("initializer"))
            .child(self.semi.tree().label("semi"))
    }
}
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct Initializer<'a> {
    pub eq: Eq<'a>,
    pub expr: Expression<'a>,
}
impl<'a> Node<'a> for LetStatement<'a> {
    fn parse(parser: &mut crate::ast::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let let_tkn = parser.step(Let::parse)?;
        let ident = parser.step(Identifier::parse)?;
        let initializer = parser.step(|parser| {
            let eq = parser.step(Eq::parse)?;
            parser.panic = true;
            let expr = parser.step(Expression::parse)?;
            parser.panic = false;
            Ok(Initializer { eq, expr })
        });
        if parser.panic {
            return Err(initializer.err().unwrap());
        }
        let semi = parser.step(SemiColon::parse)?;
        Ok(Self {
            let_tkn,
            ident,
            initializer: initializer.ok(),
            semi,
        })
    }
}
