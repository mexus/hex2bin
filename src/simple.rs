use super::{Calculator, Chunks};
use std::io::{self, Write};

pub fn copy<W>(
    input_bytes: &[u8],
    mut output: W,
    reading_chunk_len: usize,
    calculator: Calculator,
) -> io::Result<()>
where
    W: Write,
{
    let writing_chunk_len = reading_chunk_len / 2;
    let mut output_buf = vec![0; writing_chunk_len];
    for reading_chunk in Chunks::new(input_bytes, reading_chunk_len) {
        let bytes_to_write = calculator.process(reading_chunk, &mut output_buf);
        output.write_all(unsafe { &output_buf.get_unchecked(..bytes_to_write) })?;
    }
    output.flush()?;
    Ok(())
}
