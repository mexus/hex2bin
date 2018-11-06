use memmap::Mmap;
use std::fs::{self, File};
use std::io;
use std::time::Instant;

mod args;
mod calculator;
mod chunks;
mod chunks_mut;
mod crossbeam;
mod simple;

use self::args::{Args, Mode};
use self::calculator::Calculator;
use self::chunks::Chunks;
use self::chunks_mut::ChunksMut;

fn main() -> io::Result<()> {
    let args = Args::init();

    let start = Instant::now();

    let input = File::open(&args.in_file)?;
    let input = unsafe { Mmap::map(&input).unwrap() };
    let output = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(&args.out_file)?;
    output.set_len(input.len() as u64 / 2)?;

    let calculator = Calculator::new(4);

    match args.mode {
        Mode::Simple => simple::copy(&input, output, args.chunk_size, calculator)?,
        Mode::Crossbeam => crossbeam::copy(&input, output, args.chunk_size, calculator)?,
    }

    let end = Instant::now();
    println!("{:?}", end - start);
    Ok(())
}
