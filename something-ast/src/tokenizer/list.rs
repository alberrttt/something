/// To you, when you had your sanity
/// this list is jankey and weird. You should rewrite it.
use super::prelude::*;

#[derive(Clone, PartialEq)]
pub struct List<T>
where
    T: Node,
{
    pub(super) items: Vec<T>,
}

impl<T> Node for List<T>
where
    T: Node + Clone + std::fmt::Debug,
{
    #[track_caller]
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self>
    where
        Self: Clone + std::fmt::Debug + Clone,
    {
        let mut list = Self::new();
        while !parser.at_end() {
            let _next = parser.peek()?.clone();
            if _next.is_closing_delimiter() {
                break;
            }
            match parser.parse() {
                Ok(item) => list.push(item),
                Err(err) => {
                    devprintln!("{}", err);
                    parser.advance();
                }
            }
        }
        Ok(list)
    }
}
impl<T> List<T>
where
    T: Node,
{
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> Default for List<T>
where
    T: Node,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> IntoIterator for List<T>
where
    T: Node,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
impl<T> std::ops::Index<usize> for List<T>
where
    T: Node,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
impl<T> std::ops::IndexMut<usize> for List<T>
where
    T: Node,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}
impl<T> std::ops::Deref for List<T>
where
    T: Node,
{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl<T> std::ops::DerefMut for List<T>
where
    T: Node,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
impl<T> std::fmt::Debug for List<T>
where
    T: Node + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.items.iter()).finish()
    }
}
