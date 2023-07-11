use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice::SliceIndex;

pub struct InlineVec<T, const N: usize> {
    len: usize,
    buffer: [MaybeUninit<T>; N],
}
impl<T, const N: usize> InlineVec<T, N> {
    pub const fn new() -> Self {
        Self {
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    pub const fn capacity(&self) -> usize {
        N
    }
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.len >= self.capacity() {
            return Err(item);
        }
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            (*ptr.add(self.len)).write(item);
        }
        self.len += 1;

        return Ok(());
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len <= 0 {
            return None;
        }

        self.len -= 1;
        let item = unsafe { self.buffer.as_mut_ptr().add(self.len).read().assume_init() };

        return Some(item);
    }

    pub fn peek(&self) -> Option<&T> {
        if self.len <= 0 {
            return None;
        }

        let item = unsafe { (&*self.buffer.as_ptr().add(self.len - 1)).assume_init_ref() };

        return Some(item);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len <= 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.len <= index {
            None
        } else {
            Some(unsafe { self.buffer[index].assume_init_ref() })
        }
    }

    pub fn items<'a>(&'a self) -> &'a [T] {
        unsafe { std::mem::transmute(&self.buffer[..self.len]) }
    }

    pub fn items_mut<'a>(&'a mut self) -> &'a mut [T] {
        unsafe { std::mem::transmute(&mut self.buffer[..self.len]) }
    }
}

impl<T, const N: usize> Drop for InlineVec<T, N> {
    fn drop(&mut self) {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            for i in 0..self.len {
                ptr.add(i).drop_in_place();
            }
        }
    }
}

impl<T, const N: usize> Deref for InlineVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.items()
    }
}

impl<T, const N: usize> DerefMut for InlineVec<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.items_mut()
    }
}

impl<I: SliceIndex<[T]>, T, const N: usize> Index<I> for InlineVec<T, N> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.items()[index]
    }
}
impl<I: SliceIndex<[T]>, T, const N: usize> IndexMut<I> for InlineVec<T, N> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.items_mut()[index]
    }
}
impl<T, const N: usize> Default for InlineVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
