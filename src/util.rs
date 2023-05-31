pub(crate) trait SliceExt {
    fn try_split_at(&self, mid: usize) -> Option<(&Self, &Self)>;
}

impl<T> SliceExt for [T] {
    fn try_split_at(&self, mid: usize) -> Option<(&Self, &Self)> {
        (mid < self.len()).then(|| self.split_at(mid))
    }
}
