use crate::{error::TypeError, typechecker::Typechecker};
pub type TypeCheckResult<'a, 'b, T> = Result<T, TypeError<'a, 'b>>;
pub trait Check<'a, 'b, Result> {
    fn check(&'b self, tc: &mut Typechecker<'a, 'b>) -> TypeCheckResult<'a, 'b, Result>;
}
