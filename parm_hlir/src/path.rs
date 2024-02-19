use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct Path<'a, 'b> {
    todo: PhantomData<(&'a &'b (),)>,
}
