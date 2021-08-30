pub type ParseResult<T> = Result<(usize, T), ()>;

fn p_ok<T>(offset: usize, result: T) -> ParseResult<T> {
    Ok((offset, result))
}

trait Parser<T>: Sized {
    type Output;
    fn parse(&self, input: &[T], offset: usize) -> ParseResult<Self::Output>;

    fn after<O, F>(self, apply: F) -> After<Self, F>
    where
        F: Fn(Self::Output) -> O,
    {
        After {
            parser: self,
            apply,
        }
    }

    fn and<P: Parser<T>>(self, other: P) -> And<Self, P> {
        And {
            first: self,
            sec: other,
        }
    }

    fn or<P: Parser<T, Output = Self::Output>>(self, other: P) -> Or<Self, P> {
        Or {
            this: self,
            or_that: other,
        }
    }
}

impl<T, O, F> Parser<T> for F
where
    F: Fn(&[T], usize) -> ParseResult<O>,
{
    type Output = O;

    fn parse(&self, input: &[T], offset: usize) -> ParseResult<O> {
        self(input, offset)
    }
}

pub struct And<P, Q> {
    first: P,
    sec: Q,
}

impl<P, Q, T> Parser<T> for And<P, Q>
where
    P: Parser<T>,
    Q: Parser<T>,
{
    type Output = (P::Output, Q::Output);

    fn parse(&self, input: &[T], offset: usize) -> ParseResult<Self::Output> {
        let (offset, first) = self.first.parse(input, offset)?;
        let (offset, sec) = self.sec.parse(input, offset)?;

        p_ok(offset, (first, sec))
    }
}

pub struct After<P, F> {
    parser: P,
    apply: F,
}

impl<P, F, T, O> Parser<T> for After<P, F>
where
    P: Parser<T>,
    F: Fn(P::Output) -> O,
{
    type Output = O;

    fn parse(&self, input: &[T], offset: usize) -> ParseResult<O> {
        let (offset, output) = self.parser.parse(input, offset)?;
        let output = (self.apply)(output);
        p_ok(offset, output)
    }
}

pub struct Or<P, Q> {
    this: P,
    or_that: Q,
}

impl<P, Q, T, O> Parser<T> for Or<P, Q>
where
    P: Parser<T, Output = O>,
    Q: Parser<T, Output = O>,
{
    type Output = O;

    fn parse(&self, input: &[T], offset: usize) -> ParseResult<Self::Output> {
        self.this
            .parse(input, offset)
            .or_else(|e| self.or_that.parse(input, offset))
    }
}

pub struct Many<P> {
    parser: P,
}

impl<P, T> Parser<T> for Many<P>
where
    P: Parser<T>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: &[T], mut offset: usize) -> ParseResult<Self::Output> {
        let vec: Vec<_> = std::iter::repeat(std::marker::PhantomData)
            .map_while(|_| {
                let result = self.parser.parse(input, offset).ok()?;
                offset = result.0;
                Some(result.1)
            })
            .collect();

        if vec.is_empty() {
            // TODO: make nice errors
            Err(())
        } else {
            p_ok(offset, vec)
        }
    }
}
