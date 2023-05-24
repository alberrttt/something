use crate::prelude::*;

pub trait TypeCheck<With = (), Returns = ()> {
    fn type_check(&self, with: With) -> Returns;
}
