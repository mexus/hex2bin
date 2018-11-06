use std::cmp::min;
use std::mem;
use std::slice;

#[inline(always)]
unsafe fn split_at_mut<'a, T>(slice: &'a mut [T], at: usize) -> (&'a mut [T], &'a mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    (
        slice::from_raw_parts_mut(ptr, at),
        slice::from_raw_parts_mut(ptr.add(at), len - at),
    )
}

pub struct ChunksMut<'a, T> {
    slice: &'a mut [T],
    chunk: usize,
}

impl<'a, T> ChunksMut<'a, T> {
    #[inline(always)]
    pub fn new(slice: &'a mut [T], chunk: usize) -> Self {
        ChunksMut { slice, chunk }
    }
}

impl<'a, T> Iterator for ChunksMut<'a, T> {
    type Item = &'a mut [T];

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let size = min(self.chunk, self.slice.len());
            let tmp = mem::replace(&mut self.slice, &mut []);
            let (first, last) = unsafe { split_at_mut(tmp, size) };
            self.slice = last;
            Some(first)
        }
    }
}
