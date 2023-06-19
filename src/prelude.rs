use std::{iter::Peekable, vec::IntoIter};

pub trait IteratorExt<I: Iterator> {
    fn take_while<F>(&mut self, predicate: F) -> IntoIter<I::Item>
    where
        F: Fn(&I::Item) -> bool;
}

impl<I: Iterator> IteratorExt<I> for Peekable<I> {
    fn take_while<F>(&mut self, predicate: F) -> IntoIter<I::Item>
    where
        F: Fn(&I::Item) -> bool,
    {
        let mut v: Vec<I::Item> = vec![];
        while self.peek().map_or(false, &predicate) {
            v.push(self.next().unwrap());
        }

        v.into_iter()
    }
}
