use super::parse_error::ParseError;
pub type ParseResult<T> = Result<(usize, T), ParseError<'static>>;

pub fn p_ok<T>(offset: usize, result: T) -> ParseResult<T> {
    Ok((offset, result))
}

/// a parser gets a list inputs and a position to start
/// and then returns the next position + an arbitrary output on success
/// or an nice error message on failure
pub trait Parser<T>: Sized {
    type Output;
    /// parses the input at the position and returns the result
    fn parse(&self, input: &[T], offset: usize) -> ParseResult<Self::Output>;

    /// change the output of the parser with a function
    fn process<O, F>(self, apply: F) -> Process<Self, F>
    where
        F: Fn(Self::Output) -> O,
    {
        Process {
            parser: self,
            apply,
        }
    }

    /// look at the output of the parser, mostly for debugging
    fn inspect<F: Fn(&Self::Output)>(self, inspector: F) -> Inspect<Self, F> {
        Inspect {
            parser: self,
            inspector,
        }
    }

    /// create a parser that parses first this one and
    /// the second one behind it. The output is a tuple of
    /// both outputs
    fn and<P: Parser<T>>(self, other: P) -> And<Self, P> {
        And {
            first: self,
            sec: other,
        }
    }

    /// make sure there is a certain suffix behind the parsed content,
    /// output remains unchanged
    fn suffix<P: Parser<T>>(self, after: P) -> Suffix<Self, P> {
        Suffix {
            parser: self,
            suffix: after,
        }
    }

    /// make sure there is a certain prefix before the parsed content,
    /// output remains unchanged
    fn prefix<P: Parser<T>>(self, after: P) -> Prefix<Self, P> {
        Prefix {
            parser: self,
            prefix: after,
        }
    }

    /// creates a new parser where either this or the other needs to parse
    /// for a valid result. Both parsers need to return the same type
    fn or<P: Parser<T, Output = Self::Output>>(self, other: P) -> Or<Self, P> {
        Or {
            this: self,
            or_that: other,
        }
    }

    /// repeat this parser (greedy)
    /// returns a Vec of the original Output
    fn many(self) -> Many<Self> {
        Many { parser: self }
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

#[derive(Clone)]
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

pub struct Suffix<P, Q> {
    parser: P,
    suffix: Q,
}

impl<P, Q, T> Parser<T> for Suffix<P, Q>
where
    P: Parser<T>,
    Q: Parser<T>,
{
    type Output = P::Output;

    fn parse(&self, input: &[T], offset: usize) -> ParseResult<Self::Output> {
        let (offset, out) = self.parser.parse(input, offset)?;
        let (offset, _) = self.suffix.parse(input, offset)?;

        p_ok(offset, out)
    }
}

#[derive(Clone)]
pub struct Process<P, F> {
    parser: P,
    apply: F,
}

impl<P, F, T, O> Parser<T> for Process<P, F>
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

#[derive(Clone)]
pub struct Prefix<P, Q> {
    parser: P,
    prefix: Q,
}

impl<P, Q, T> Parser<T> for Prefix<P, Q>
where
    P: Parser<T>,
    Q: Parser<T>,
{
    type Output = P::Output;

    fn parse(&self, input: &[T], mut offset: usize) -> ParseResult<Self::Output> {
        offset = self.prefix.parse(input, offset)?.0;
        self.parser.parse(input, offset)
    }
}

#[derive(Clone)]
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
            .or_else(|_| self.or_that.parse(input, offset))
    }
}

#[derive(Clone)]
pub struct Many<P> {
    parser: P,
}

impl<P, T> Parser<T> for Many<P>
where
    P: Parser<T>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: &[T], mut offset: usize) -> ParseResult<Self::Output> {
        let mut error = None;
        let vec: Vec<_> =
            // creates an infinite iterator with nothing
            std::iter::repeat(std::marker::PhantomData)
            // map it to every valid parse result
            .map_while(|_: std::marker::PhantomData<()>| {
                let result = self
                    .parser
                    .parse(input, offset)
                    // save the error
                    .map_err(|e| error = Some(e))
                    .ok()?;

                offset = result.0;
                Some(result.1)
            })
            .collect();

        // couldn't parse anything => return Error
        if vec.is_empty() {
            // should be safe
            Err(error.unwrap())
        } else {
            p_ok(offset, vec)
        }
    }
}

#[derive(Clone)]
pub struct Inspect<P, F> {
    parser: P,
    inspector: F,
}

impl<P, F, T> Parser<T> for Inspect<P, F>
where
    P: Parser<T>,
    F: Fn(&P::Output),
{
    type Output = P::Output;

    fn parse(&self, input: &[T], offset: usize) -> ParseResult<Self::Output> {
        let result = self.parser.parse(input, offset)?;
        (self.inspector)(&result.1);
        Ok(result)
    }
}

/// parser to check if this is already the end of file
pub fn eof<T: std::fmt::Debug>(input: &[T], offset: usize) -> ParseResult<()> {
    if offset >= input.len() {
        p_ok(offset, ())
    } else {
        Err(ParseError {
            expected: "EOF",
            actual: format!("{:?}", input[offset]),
        })
    }
}
