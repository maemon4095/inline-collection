use std::collections::VecDeque;

pub struct AnyPeekable<I: Iterator> {
    iter: I,
    buffer: VecDeque<I::Item>,
}

impl<I: Iterator> From<I> for AnyPeekable<I> {
    fn from(iter: I) -> Self {
        Self {
            iter,
            buffer: VecDeque::new(),
        }
    }
}

impl<I: Iterator> AnyPeekable<I> {
    #[allow(unused)]
    fn peek(&mut self) -> Option<&I::Item> {
        self.nth_peek(0)
    }
    #[allow(unused)]
    fn nth_peek(&mut self, index: usize) -> Option<&I::Item> {
        self.peek_any().nth(index)
    }

    fn peek_any<'a>(&'a mut self) -> AnyIter<'a, I> {
        AnyIter {
            peekable: self,
            index: Some(0),
        }
    }
}

impl<I: Iterator> Iterator for AnyPeekable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop_front().or_else(|| self.iter.next())
    }
}

pub struct AnyIter<'a, I: Iterator> {
    peekable: &'a mut AnyPeekable<I>,
    index: Option<usize>,
}

impl<'a, I: Iterator> Iterator for AnyIter<'a, I> {
    type Item = &'a I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let buffer = &mut self.peekable.buffer;
        if self.index.is_some() && buffer.len() > self.index.unwrap() {
            let ret = buffer.get(self.index.unwrap());
            self.index = Some(self.index.unwrap() + 1);
            return unsafe { std::mem::transmute(ret) }; //the peekable field lives longer than self
        }
        self.index = None;
        self.peekable.iter.next().and_then(|e| {
            buffer.push_back(e);
            unsafe { std::mem::transmute(buffer.back()) }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::iterators::IteratorExtensions;

    #[test]
    fn peek_any() {
        validate([1, 2, 3, 4, 5]);
        validate([0; 0]);
        validate([1]);

        fn validate<T: Copy + PartialEq + Debug, const N: usize>(items: [T; N]) {
            let clone = items.clone();
            let mut peekable = items.iter().any_peekable();
            assert!(peekable.peek_any().cloned().eq(clone.iter()));
            assert!(peekable.eq(clone.iter()));
        }
    }
}
