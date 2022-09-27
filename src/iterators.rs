mod npeekable;
mod vec_peekable;
pub use npeekable::NPeekable;
pub use vec_peekable::VecPeekable;
pub trait IteratorExtensions<I: Iterator> {
    fn n_peekable<const N: usize>(self) -> NPeekable<I, N>;
    fn vec_peekable(self) -> VecPeekable<I>;
}

impl<I: Iterator> IteratorExtensions<I> for I {
    fn n_peekable<const N: usize>(self) -> NPeekable<I, N> {
        NPeekable::from(self)
    }

    fn vec_peekable(self) -> VecPeekable<I> {
        todo!()
    }
}
