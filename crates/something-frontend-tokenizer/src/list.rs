use crate::{
    error::ParseError,
    traits::{AppendTokens, ParsingDisplay},
    Parse,
};

#[derive(Clone, PartialEq)]
pub struct List<T>
where
    T: Parse,
{
    pub(super) items: Vec<T>,
}
impl<T> AppendTokens for List<T>
where
    T: Parse + AppendTokens,
{
    fn append_tokens(&self, tokens: &mut crate::Tokens)
    where
        Self: Sized,
    {
        for item in &self.items {
            item.append_tokens(tokens);
        }
    }
}
impl<T> ParsingDisplay for List<T>
where
    T: Parse,
{
    fn display(&self) -> String
    where
        Self: Sized,
    {
        self.items
            .iter()
            .map(|item| item.display())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn placeholder() -> String
    where
        Self: Sized,
    {
        format!("[...<{}>...]", T::placeholder())
    }
}
impl<T> Parse for List<T>
where
    T: Parse + Clone + std::fmt::Debug,
{
    fn parse(input: &mut crate::Tokens) -> Result<Self, ParseError>
    where
        Self: Clone + std::fmt::Debug + Clone,
    {
        let mut list = Self::new();
        while !input.at_end() {
            list.push(input.parse()?);
        }
        Ok(list)
    }
}
impl<T> List<T>
where
    T: Parse,
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
    T: Parse,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> IntoIterator for List<T>
where
    T: Parse,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
impl<T> std::ops::Index<usize> for List<T>
where
    T: Parse,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
impl<T> std::ops::IndexMut<usize> for List<T>
where
    T: Parse,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}
impl<T> std::ops::Deref for List<T>
where
    T: Parse,
{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl<T> std::ops::DerefMut for List<T>
where
    T: Parse,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
impl<T> std::fmt::Debug for List<T>
where
    T: Parse + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.items.iter()).finish()
    }
}
