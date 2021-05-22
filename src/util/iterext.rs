use super::advancer::Advancer;
use super::remove_first::RemoveFirst;
use std::iter::Peekable;

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

/*pub trait IterExt<I>: Iterator
where
    I: Iterator,
    I::Item: PartialEq,
{
    fn leave_one(self, filter: I::Item) -> RemoveFirst<I>;
}

impl<I> IterExt<I> for I
where
    I: Iterator,
    I::Item: PartialEq,
{
    fn leave_one(self, filter: I::Item) -> RemoveFirst<I> {
        RemoveFirst::new(self, filter)
    }
}*/
