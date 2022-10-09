#![cfg(any())]
use std::{marker::PhantomData, mem::MaybeUninit, ptr::NonNull};

pub struct Sequence<T, S: Strategy> {
    head: Option<NonNull<Segment<T>>>,
    tail: Option<NonNull<Segment<T>>>,
    strategy: S,
    len: usize,
}

impl<T, S: Strategy> Sequence<T, S> {
    fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            segment: self.head,
            index: 0,
            marker: PhantomData,
        }
    }

    fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            segment: self.head,
            index: 0,
            marker: PhantomData,
        }
    }

    fn append(&mut self, item: T) {}

    fn prepend(&mut self, item: T) {}
}

pub struct Cursor<'a, T, S: Strategy> {
    current: &'a mut Option<NonNull<Segment<T>>>,
    strategy: &'a S,
    index: usize,
}

pub trait Strategy {
    fn next_size(&mut self, segment_size: usize, required_capacity: usize) -> usize;
}

impl<T: FnMut(usize, usize) -> usize> Strategy for T {
    fn next_size(&mut self, segment_size: usize, required_capacity: usize) -> usize {
        (self)(segment_size, required_capacity)
    }
}

struct Segment<T> {
    next: Option<NonNull<Segment<T>>>,
    back: Option<NonNull<Segment<T>>>,
    len: usize,
    buffer: [MaybeUninit<T>],
}

impl<T> Segment<T> {
    fn insert<const N: usize>(
        &mut self,
        strategy: &mut impl Strategy,
        index: usize,
        items: impl ExactSizeIterator<Item = T>,
    ) {
        let req = items.len().checked_sub(self.free_len());
        if req.map_or(false, |r| r > 0) { //insert item len exceeds free len
        } else {
        }
    }

    fn free_len(&self) -> usize {
        self.buffer.len() - self.len
    }

    fn items<'a>(&'a self) -> &'a [T] {
        unsafe { std::mem::transmute(&self.buffer[..self.len]) }
    }

    fn items_mut<'a>(&'a mut self) -> &'a mut [T] {
        unsafe { std::mem::transmute(&mut self.buffer[..self.len]) }
    }
}

struct Iter<'a, T: 'a> {
    segment: Option<NonNull<Segment<T>>>,
    index: usize,
    marker: PhantomData<&'a Segment<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.segment.is_none() {
            return None;
        }
        let node = unsafe { self.segment.unwrap().as_ref() };
        {
            let items = node.items();
            if self.index < items.len() {
                let ret = &items[self.index];
                self.index += 1;
                return Some(ret);
            }
        }

        self.segment = node.next;
        self.index = 0;
        return self.next();
    }
}

struct IterMut<'a, T: 'a> {
    segment: Option<NonNull<Segment<T>>>,
    index: usize,
    marker: PhantomData<&'a mut Segment<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.segment.is_none() {
            return None;
        }
        {
            let items = unsafe { self.segment.unwrap().as_mut().items_mut() };
            if self.index < items.len() {
                let ret = &mut items[self.index];
                self.index += 1;
                return Some(ret);
            }
        }

        self.segment = unsafe { self.segment.unwrap().as_mut().next };
        self.index = 0;
        return self.next();
    }
}
