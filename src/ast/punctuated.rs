use std::ops::Index;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Punctuated<T, U> {
    pub(crate) items: Vec<T>,
    pub(crate) punct: Vec<U>,
}

impl<T, U> Punctuated<T, U> {
    pub const fn new() -> Self {
        Self {
            items: Vec::new(),
            punct: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T, punct: U) {
        self.items.push(item);
        self.punct.push(punct);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn last(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn iter_punct(&self) -> impl Iterator<Item = &U> {
        self.punct.iter()
    }
}

impl<T, U> Index<usize> for Punctuated<T, U> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
