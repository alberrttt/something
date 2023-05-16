use std::slice::Iter;

pub trait Children<Child> {
    fn children(&self) -> Iter<Child>;
}

pub trait Name: std::fmt::Debug {
    fn name() -> &'static str
    where
        Self: Sized;
    fn named(&self) -> &'static str;
}
