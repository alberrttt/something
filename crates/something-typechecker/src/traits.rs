use crate::prelude::*;

pub trait TypeCheck<With = (), Returns = Result<(), TypeError>> {
    fn type_check(&self, with: With) -> Returns;
}
