#[macro_export]
macro_rules! map {
    ($($key:expr => $val:expr),*$(,)?) => {{
        use std::iter::FromIterator;
        ::std::collections::HashMap::from_iter([ $(($key.into(), $val),)* ])
    }};
}
