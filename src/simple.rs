use super::{Calculator, Chunks};
use std::io::{self, Write};

pub fn copy<W>(
    input_bytes: &[u8],
    mut output: W,
    write_chunk_size: usize,
    calculator: Calculator,
) -> io::Result<()>
where
    W: Write,
{
    let mut output_buf = vec![0; write_chunk_size];
    for reading_chunk in Chunks::new(input_bytes, write_chunk_size) {
        calculator.process(reading_chunk, &mut output_buf);
        output.write_all(unsafe { &output_buf.get_unchecked(..reading_chunk.len() / 2) })?;
    }
    output.flush()?;
    Ok(())
}
