use crate::types::sig::TypeSig;

pub trait ResolveType<To = TypeSig> {
    type Context;
    fn resolve(&self, ctx: &mut Self::Context) -> To;
}
