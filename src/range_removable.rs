pub trait RangeRemovable<T> {
    fn swap_remove_all(&mut self, predicate: impl FnMut(&T) -> bool, removed: impl FnMut(T));
    fn remove_all(&mut self, predicate: impl FnMut(&T) -> bool, removed: impl FnMut(T));
}

impl<T> RangeRemovable<T> for Vec<T> {
    fn swap_remove_all(
        &mut self,
        mut predicate: impl FnMut(&T) -> bool,
        mut removed: impl FnMut(T),
    ) {
        let mut rest = self.len();
        while let Some(idx) = self[..rest].iter().rposition(&mut predicate) {
            removed(self.swap_remove(idx));
            rest = idx;
        }
    }

    fn remove_all(&mut self, mut predicate: impl FnMut(&T) -> bool, mut removed: impl FnMut(T)) {
        let ptr = self.as_mut_ptr();
        let len = self.len();
        let mut removed_count = 0;
        unsafe {
            for index in 0..len {
                let p = ptr.add(index);
                if predicate(&*p) {
                    removed(p.read());
                    removed_count += 1;
                } else if removed_count > 0 {
                    std::ptr::copy_nonoverlapping(p, p.sub(removed_count), 1);
                }
            }

            self.set_len(len - removed_count);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::range_removable::RangeRemovable;
    use crate::testing::*;
    use std::{borrow::Borrow, fmt::Debug};

    #[test]
    fn swap_remove_all() {
        validate([1, 2, 3], |i| i % 2 == 0);
        validate([1, 2, 3, 4, 5, 6], |i| i % 2 == 0);

        fn validate<T, const N: usize>(source: [T; N], mut pred: impl Clone + FnMut(&T) -> bool) {
            let watcher = DropWatcher::new();
            {
                let mut vec = Vec::new();
                for x in source {
                    vec.push(watcher.alloc(x));
                }

                let predicate = {
                    let mut p = pred.clone();
                    move |e: &DropMarker<T>| p(e.props().borrow())
                };

                vec.swap_remove_all(predicate, |e| assert!(pred(e.props().borrow())));
            }

            assert!(watcher.markers().iter().all(|e| e.is_properly_dropped()));
        }
    }

    #[test]
    fn remove_all() {
        validate([1, 2, 3], |i| i % 2 == 0);
        validate([1, 2, 3, 4, 5, 6], |i| i % 2 == 0);

        fn validate<T: Debug, const N: usize>(
            source: [T; N],
            mut pred: impl Clone + FnMut(&T) -> bool,
        ) {
            let watcher = DropWatcher::new();
            {
                let mut vec = Vec::new();
                for x in source {
                    vec.push(watcher.alloc(x));
                }

                let predicate = {
                    let mut p = pred.clone();
                    move |e: &DropMarker<T>| p(e.props().borrow())
                };

                vec.remove_all(predicate, |e| assert!(pred(e.props().borrow())));
            }

            assert!(watcher.markers().iter().all(|e| e.is_properly_dropped()));
        }
    }
}
