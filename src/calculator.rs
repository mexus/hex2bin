use super::{Chunks, ChunksMut};
use rayon::{ThreadPool, ThreadPoolBuildError, ThreadPoolBuilder};

pub struct Calculator {
    thread_pool: ThreadPool,
    thread_count: usize,
}

#[inline(always)]
fn hex_to_dec(s: u8) -> u8 {
    match s {
        b'0'..=b'9' => s - b'0',
        b'a'..=b'f' => 0xa + (s - b'a'),
        b'A'..=b'F' => 0xa + (s - b'A'),
        _ => panic!("Unexpected char #{}", s),
    }
}

#[inline(always)]
fn calculate(input: &[u8], output: &mut [u8]) {
    for i in 0..output.len() {
        unsafe {
            let bin = hex_to_dec(*input.get_unchecked(2 * i)) << 4
                | hex_to_dec(*input.get_unchecked(2 * i + 1));
            *output.get_unchecked_mut(i) = bin;
        }
    }
}

impl Calculator {
    pub fn new(threads: usize) -> Result<Self, ThreadPoolBuildError> {
        Ok(Calculator {
            thread_pool: ThreadPoolBuilder::new().num_threads(threads).build()?,
            thread_count: threads,
        })
    }

    #[inline]
    pub fn process(&self, reading_chunk: &[u8], output: &mut [u8]) {
        let writing_chunk = unsafe { output.get_unchecked_mut(..reading_chunk.len() / 2) };
        let input_per_thread = Chunks::new(reading_chunk, reading_chunk.len() / self.thread_count);
        let output_per_thread =
            ChunksMut::new(writing_chunk, writing_chunk.len() / self.thread_count);
        let per_thread = input_per_thread.zip(output_per_thread);

        self.thread_pool.scope(|scope| {
            for (input, output) in per_thread {
                scope.spawn(move |_| calculate(input, output));
            }
        });
    }
}
