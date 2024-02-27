use crate::parser::nodes::visibility::Visibility;

use super::*;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct ModDeclaration<'a> {
    pub visibility: Visibility<'a>,
    pub mod_tkn: ModKw<'a>,
    pub name: Identifier<'a>,
    pub category: ModuleDeclarationType<'a>,
}
impl<'a> Node<'a, ParseResult<'a, Self>> for ModDeclaration<'a> {
    fn parse(parse_stream: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let visibility = parse_stream.step(Visibility::parse)?;
        let mod_tkn = parse_stream.step(ModKw::parse)?;
        parse_stream.panic = true;
        let name = parse_stream.step(Identifier::parse)?;
        let category = if let Ok(semicolon) = parse_stream.step(SemiColon::parse) {
            ModuleDeclarationType::File(semicolon)
        } else {
            todo!()
        };
        parse_stream.panic = false;
        Ok(Self {
            visibility,
            mod_tkn,
            name,
            category,
        })
    }
}
impl<'a> TreeDisplay for ModDeclaration<'a> {
    fn tree(&self) -> Tree {
        Tree::new("ModDeclaration")
            .child(self.mod_tkn.tree().label("mod_tkn"))
            .child(self.name.tree().label("name"))
            .child(match self.category {
                ModuleDeclarationType::File(ref s) => Tree::new("file"),
                ModuleDeclarationType::InlineBody(ref b) => b.body.tree(),
            })
    }
}

#[derive(Debug, Clone, PartialEq, Spanned)]
pub enum ModuleDeclarationType<'a> {
    /// pub mod foo;
    File(SemiColon<'a>),
    /// pub mod foo { ... }
    InlineBody(ModBody<'a>),
}

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct ModBody<'a> {
    pub body: Brace<'a, Vec<Item<'a>>>,
}
