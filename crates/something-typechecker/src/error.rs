use crate::prelude::Type;

pub enum TypeError {
    MismatchedTypes { expected: Type, found: Type },
}
