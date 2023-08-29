use crate::symbol::Type;

pub trait TypeCheck {
    type Output;
    type With;
    type Against = Option<Type>;
    fn type_check(&self, with: Self::With, against: Self::Against) -> Self::Output;
}
