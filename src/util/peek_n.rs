use std::fmt::Debug;
pub struct PeekN<I: Iterator> {
    peeked: Vec<I::Item>,
    iter: I,
}

impl<I: Iterator> PeekN<I>
where
    I::Item: Debug,
{
    pub fn new(iter: I) -> Self {
        PeekN {
            peeked: Vec::new(),
            iter,
        }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    pub fn peek_nth(&mut self, nth: usize) -> Option<&I::Item> {
        for _ in self.peeked.len()..=nth {
            self.peeked.push(self.iter.next()?);
        }

        self.peeked.get(self.peeked.len() - nth - 1)
    }

    pub fn next_if<P>(&mut self, pred: P) -> Option<I::Item>
    where
        P: FnOnce(&I::Item) -> bool,
    {
        if pred(self.peek()?) {
            self.next()
        } else {
            None
        }
    }
}

impl<I: Iterator> Iterator for PeekN<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        //self.peeked.().or_else(|| self.iter.next())
        if self.peeked.len() > 0 {
            Some(self.peeked.remove(0))
        } else {
            self.iter.next()
        }
    }
}
