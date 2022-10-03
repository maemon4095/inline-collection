use std::fmt;
struct Vector<T, const N: usize> {
    buffer: [T; N],
}

impl<T, const N: usize> fmt::Debug for Vector<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("buffer", &self.buffer)
            .finish()
    }
}
