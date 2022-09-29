mod any_peekable;
mod npeekable;
pub use any_peekable::AnyPeekable;
pub use npeekable::NPeekable;
pub trait IteratorExtensions<I: Iterator> {
    fn n_peekable<const N: usize>(self) -> NPeekable<I, N>;
    fn any_peekable(self) -> AnyPeekable<I>;
}

impl<I: Iterator> IteratorExtensions<I> for I {
    fn n_peekable<const N: usize>(self) -> NPeekable<I, N> {
        NPeekable::from(self)
    }

    fn any_peekable(self) -> AnyPeekable<I> {
        AnyPeekable::from(self)
    }
}
