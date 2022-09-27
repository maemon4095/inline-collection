use std::collections::VecDeque;

pub struct VecPeekable<I: Iterator> {
    iter: I,
    buffer: VecDeque<I::Item>,
}
//TODO: not only peek api but also impl rewind api.
//rewind api allows reuse iterator api
