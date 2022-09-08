pub(crate) use memory_testing_utils::*;

pub(crate) trait IsSomeAndExtension<T> {
    fn any(&self, predicate: impl FnOnce(&T) -> bool) -> bool;
}

impl<T> IsSomeAndExtension<T> for Option<T> {
    fn any(&self, predicate: impl FnOnce(&T) -> bool) -> bool {
        self.as_ref().map_or(false, predicate)
    }
}
