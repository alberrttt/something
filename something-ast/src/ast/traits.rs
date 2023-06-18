use std::slice::Iter;

pub trait Children<Child> {
    fn children(&self) -> Iter<Child>;
}
