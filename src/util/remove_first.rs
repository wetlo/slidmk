pub struct RemoveFirst<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    pub iter: I,
    pub to_remove: I::Item,
    //pub already_there: bool,
}

impl<I> Iterator for RemoveFirst<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let n = self.iter.next()?;

        if n == self.to_remove {
            self.iter.next()
        } else {
            //self.already_there = false;
            Some(n)
        }
    }
}
