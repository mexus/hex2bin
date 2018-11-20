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
}

#[inline(always)]
fn worker<W: Write>(receiver: &Receiver<Commands>, mut internal: Vec<u8>, mut writer: W) {
    while let Ok(cmd) = receiver.recv() {
        match cmd {
            Commands::Write { len: 0, .. } => break,
            Commands::Write {
                mut data,
                len,
                sender,
            } => {
                mem::swap(&mut data, &mut internal);
                sender.send(data).unwrap();
                let buf = unsafe { internal.get_unchecked(..len) };
                writer.write_all(buf).unwrap();
            }
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
    let writing_chunk_len = reading_chunk_len / 2;
    let remote_buffer = vec![0; writing_chunk_len];
    let mut local_buffer = remote_buffer.clone();

    let worker_thread;
    {
        let (thread_sender, thread_receiver) = bounded(0);
        worker_thread = thread::spawn(move || worker(&thread_receiver, remote_buffer, output));

        let (sender, receiver) = bounded(0);
        for reading_chunk in Chunks::new(input, reading_chunk_len) {
            let bytes_to_write = calculator.process(reading_chunk, &mut local_buffer);
            thread_sender
                .send(Commands::Write {
                    data: local_buffer,
                    len: bytes_to_write,
                    sender: sender.clone(),
                })
                .unwrap();
            local_buffer = receiver.recv().unwrap();
        }
    }
    worker_thread.join().unwrap();
    Ok(())
}
