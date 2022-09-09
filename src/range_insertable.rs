use std::mem::MaybeUninit;

pub trait RangeInsertable<T> {
    fn insert_range(&mut self, index: usize, items: impl ExactSizeIterator<Item = T>);
    unsafe fn reserve_range(&mut self, index: usize, len: usize) -> &mut [MaybeUninit<T>];
}

impl<T> RangeInsertable<T> for Vec<T> {
    fn insert_range(&mut self, index: usize, items: impl ExactSizeIterator<Item = T>) {
        let reserved = unsafe { self.reserve_range(index, items.len()) };
        for (i, e) in items.enumerate() {
            reserved[i].write(e);
        }
    }

    unsafe fn reserve_range(&mut self, index: usize, len: usize) -> &mut [MaybeUninit<T>] {
        self.reserve(len);
        let ptr = self.as_mut_ptr();
        if index > self.len() {
            panic!("index out of range");
        }
        if len <= 0 {
            return &mut [];
        }

        for idx in (index..self.len()).rev() {
            let p = ptr.add(idx);
            std::ptr::copy_nonoverlapping(p, p.add(len), 1);
        }
        self.set_len(self.len() + len);
        std::slice::from_raw_parts_mut(ptr.add(index).cast::<MaybeUninit<T>>(), len)
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, mem::MaybeUninit};

    use crate::range_insertable::RangeInsertable;

    #[test]
    fn reserve_range() {
        validate(&[1, 2, 3, 4, 5], 0, 0, 4);
        validate(&[1, 2, 3, 4, 5], 0, 0, 8);
        validate(&[1, 2, 3, 4, 5], 0, 3, 8);
        validate(&[1, 2, 3, 4, 5], 0, 5, 8);

        fn validate<T: PartialEq + Clone + Debug>(src: &[T], default: T, index: usize, len: usize) {
            let mut v = src.to_vec();
            unsafe {
                let reserved = v.reserve_range(index, len);
                reserved.fill_with(|| MaybeUninit::new(default.clone()));

                v.iter().for_each(|e| print!("{:?} ", e));
                println!();

                assert!(v.iter().skip(index).take(len).all(|e| e == &default));

                assert!(v
                    .iter()
                    .take(index)
                    .chain(v.iter().skip(index + len))
                    .eq(src.iter()));
            }
            assert!(v.len() == src.len() + len);
        }
    }

    #[test]
    fn insert_range() {
        validate(&[1, 2, 3, 4, 5], &[-1, -2, -3], 0);
        validate(&[1, 2, 3, 4, 5], &[-1], 0);
        validate(&[1, 2, 3, 4, 5], &[-1, -2, -3], 3);
        validate(&[1, 2, 3, 4, 5], &[-1, -2, -3, -4], 5);
        validate(&[1, 2, 3, 4, 5], &[-1, -2, -3, -4, -5, -6], 3);

        fn validate<T: PartialEq + Clone + Debug>(src: &[T], items: &[T], index: usize) {
            let mut v = src.to_vec();
            v.insert_range(index, items.iter().map(|e| e.clone()));

            v.iter().for_each(|e| print!("{:?} ", e));
            println!();

            assert!(v.iter().skip(index).take(items.len()).eq(items.iter()));
            assert!(v
                .iter()
                .take(index)
                .chain(v.iter().skip(index + items.len()))
                .eq(src.iter()));
        }
    }
}
