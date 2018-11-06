use crossbeam_channel::{bounded, Receiver, Sender};
use std::io::{self, Write};
use std::mem;
use std::thread;

use super::{Calculator, Chunks};

enum Commands {
    Write {
        data: Vec<u8>,
        len: usize,
        sender: Sender<Vec<u8>>,
    },
    Quit,
}

#[inline(always)]
fn worker<W: Write>(receiver: &Receiver<Commands>, mut internal: Vec<u8>, mut writer: W) {
    loop {
        match receiver.recv().unwrap() {
            Commands::Write { len: 0, .. } => break,
            Commands::Write {
                mut data,
                len,
                sender,
            } => {
                mem::swap(&mut data, &mut internal);
                sender.send(data).unwrap();
                unsafe {
                    writer.write_all(internal.get_unchecked(..len)).unwrap();
                }
            }
            Commands::Quit => break,
        }
    }
}

#[inline(always)]
pub fn copy<W>(
    input: &[u8],
    output: W,
    reading_chunk_len: usize,
    calculator: Calculator,
) -> io::Result<()>
where
    W: Write + Send + 'static,
{
    let writing_chunk_len = calculator.predict_writing_chunk_size(reading_chunk_len);
    let remote_buffer = vec![0; writing_chunk_len];
    let mut local_buffer = remote_buffer.clone();

    let (thread_sender, thread_receiver) = bounded(0);
    let t = thread::spawn(move || worker(&thread_receiver, remote_buffer, output));

    for reading_chunk in Chunks::new(input, reading_chunk_len) {
        let bytes_to_write = calculator.process(reading_chunk, &mut local_buffer);
        let (sender, receiver) = bounded(0);
        thread_sender
            .send(Commands::Write {
                data: local_buffer,
                len: bytes_to_write,
                sender,
            })
            .unwrap();
        local_buffer = receiver.recv().unwrap();
    }
    thread_sender.send(Commands::Quit).unwrap();
    t.join().unwrap();
    Ok(())
}
