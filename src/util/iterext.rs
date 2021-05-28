use super::advancer::Advancer;
use super::peek_n::PeekN;
use super::remove_first::RemoveFirst;
use std::fmt::Debug;

pub trait CreateAdvancer<I: Iterator>: Iterator {
    fn advance_while<P>(&'_ mut self, predicate: P) -> Advancer<'_, I, P>
    where
        P: FnMut(&I::Item) -> bool;
}

impl<I: Iterator> CreateAdvancer<I> for PeekN<I> {
    /// advance the peekable while a certain condition is true
    /// all advanced objects can be get through the returned iterator
    fn advance_while<P>(&'_ mut self, predicate: P) -> Advancer<'_, I, P>
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

pub trait IterExt<I>: Iterator
where
    I: Iterator,
    I::Item: PartialEq,
{
    fn leave_one(self, filter: I::Item) -> RemoveFirst<I>;
    fn peekable_n(self) -> PeekN<I>;
}

impl<I> IterExt<I> for I
where
    I: Iterator,
    I::Item: PartialEq + Debug,
{
    fn leave_one(self, filter: I::Item) -> RemoveFirst<I> {
        RemoveFirst::new(self, filter)
    }

    fn peekable_n(self) -> PeekN<I> {
        PeekN::new(self)
    }
}
