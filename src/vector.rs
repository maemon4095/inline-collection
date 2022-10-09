use std::fmt::{self, Debug, Display};
pub struct Vector<T, const N: usize> {
    buffer: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    pub fn new(buffer: [T; N]) -> Self {
        Self { buffer }
    }
}

impl<T: Debug, const N: usize> fmt::Debug for Vector<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_tuple("Vector");
        for e in self.buffer[..].iter() {
            debug.field(e);
        }
        debug.finish()
    }
}

impl<T: Display, const N: usize> fmt::Display for Vector<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        if let [first, rest @ ..] = &self.buffer[..] {
            write!(f, "{}", first)?;
            for e in rest {
                write!(f, ",{}", e)?;
            }
        }

        write!(f, ")")
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Vector<T, N> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Vector<T, N> {
    type Item = &'a mut T;
    type IntoIter = ::std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter_mut()
    }
}
