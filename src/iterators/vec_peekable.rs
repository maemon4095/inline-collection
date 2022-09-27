use std::collections::VecDeque;

pub struct VecPeekable<I: Iterator> {
    iter: I,
    buffer: VecDeque<I::Item>,
}
//TODO: not only peek api but also impl rewind api.
//rewind api allows reuse iterator api

impl<I: Iterator> VecPeekable<I> {
    fn peek(&mut self) -> Option<I::Item> {
        self.nth_peek(0)
    }

    fn nth_peek(&mut self, index: usize) -> Option<I::Item> {}

    fn peek_any(&mut self) -> impl Iterator {}
}
