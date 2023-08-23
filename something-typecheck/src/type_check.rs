use crate::symbol::Type;

trait TypeCheck {
    type Output;
    type Against = Option<Type>;
    fn type_check(&self) -> Self::Output;
}
