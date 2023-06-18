use crate::{error::TypeError, types::sig::TypeSig};

pub trait ResolveType<To = TypeSig, Err = TypeError> {
    type Context;
    fn resolve(&self, ctx: &mut Self::Context) -> Result<To, Err>;
}
