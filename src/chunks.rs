use std::cmp::min;

pub struct Chunks<'a, T> {
    slice: &'a [T],
    chunk: usize,
}

impl<'a, T> Chunks<'a, T> {
    #[inline(always)]
    pub fn new(slice: &'a [T], chunk: usize) -> Self {
        Chunks { slice, chunk }
    }
}

#[inline(always)]
unsafe fn split_at<'a, T>(slice: &'a [T], at: usize) -> (&'a [T], &'a [T]) {
    (slice.get_unchecked(..at), slice.get_unchecked(at..))
}

impl<'a, T> Iterator for Chunks<'a, T> {
    type Item = &'a [T];

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let size = min(self.chunk, self.slice.len());
            let (first, last) = unsafe { split_at(self.slice, size) };
            self.slice = last;
            Some(first)
        }
    }
}
