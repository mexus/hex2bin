use super::{Chunks, ChunksMut};
use crossbeam_utils::thread::scope;

pub struct Calculator {
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
fn calculate_hex_to_bin(input: &[u8], output: &mut [u8]) {
    for i in 0..output.len() {
        unsafe {
            let bin = hex_to_dec(*input.get_unchecked(2 * i)) << 4
                | hex_to_dec(*input.get_unchecked(2 * i + 1));
            *output.get_unchecked_mut(i) = bin;
        }
    }
}

impl Calculator {
    pub fn new(threads: usize) -> Self {
        Calculator {
            thread_count: threads,
        }
    }

    #[inline]
    pub fn process(&self, reading_chunk: &[u8], output: &mut [u8]) -> usize {
        let writing_chunk_len = reading_chunk.len() / 2;
        let writing_chunk = unsafe { output.get_unchecked_mut(..writing_chunk_len) };

        let input_per_thread = Chunks::new(reading_chunk, reading_chunk.len() / self.thread_count);
        let output_per_thread =
            ChunksMut::new(writing_chunk, writing_chunk.len() / self.thread_count);
        let per_thread = input_per_thread.zip(output_per_thread);

        scope(|scope| {
            for (input, output) in per_thread {
                scope.spawn(move || calculate_hex_to_bin(input, output));
            }
        });
        writing_chunk_len
    }
}
