use std::iter::Peekable;

/// advances a mutable referance to Iterator that is
/// wrapped in a Peekable while a certain condition is true
pub struct Advancer<'a, I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    pub inner: &'a mut Peekable<I>,
    pub predicate: P,
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
