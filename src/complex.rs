use std::io::{self, Write};
use std::mem;
use std::sync::mpsc;
use std::thread;

use super::{Calculator, Chunks};

enum Commands {
    Write {
        data: Vec<u8>,
        len: usize,
        sender: mpsc::Sender<Vec<u8>>,
    },
    Quit,
}

#[inline(always)]
fn worker<W: Write>(receiver: &mpsc::Receiver<Commands>, mut internal: Vec<u8>, mut writer: W) {
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
    write_chunk_size: usize,
    calculator: Calculator,
) -> io::Result<()>
where
    W: Write + Send + 'static,
{
    let read_chunk = write_chunk_size * 2;
    let remote_buffer = vec![0; write_chunk_size];
    let mut local_buffer = remote_buffer.clone();

    let (thread_sender, thread_receiver) = mpsc::channel();
    let t = thread::spawn(move || worker(&thread_receiver, remote_buffer, output));

    for reading_chunk in Chunks::new(input, read_chunk) {
        calculator.process(reading_chunk, &mut local_buffer);
        let (sender, receiver) = mpsc::channel();
        thread_sender
            .send(Commands::Write {
                data: local_buffer,
                len: reading_chunk.len() / 2,
                sender,
            })
            .unwrap();
        local_buffer = receiver.recv().unwrap();
    }
    thread_sender.send(Commands::Quit).unwrap();
    t.join().unwrap();
    Ok(())
}
