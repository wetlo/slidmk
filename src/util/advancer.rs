use std::iter::Peekable;

/// advances a mutable referance to Iterator that is
/// wrapped in a Peekable while a certain condition is true
pub struct Advancer<'a, I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    inner: &'a mut Peekable<I>,
    predicate: P,
}

impl<'a, I, P> Iterator for Advancer<'a, I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if (self.predicate)(self.inner.peek()?) {
            self.inner.next()
        } else {
            None
        }
    }
}

pub trait CreateAdvancer<I: Iterator>: Iterator {
    fn advance_while<'a, P>(&'a mut self, predicate: P) -> Advancer<'a, I, P>
    where
        P: FnMut(&I::Item) -> bool;
}

impl<I: Iterator> CreateAdvancer<I> for Peekable<I> {
    /// advance the peekable while a certain condition is true
    /// all advanced objects can be get through the returned iterator
    fn advance_while<'a, P>(&'a mut self, predicate: P) -> Advancer<'a, I, P>
    where
        P: FnMut(&I::Item) -> bool,
        I: Iterator,
    {
        Advancer {
            inner: self,
            predicate,
        }
    }
}
