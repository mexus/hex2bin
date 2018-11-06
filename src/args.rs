use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Clone, Copy, StructOpt)]
#[structopt(name = "mode")]
pub enum Mode {
    #[structopt(name = "simple")]
    Simple,
    #[structopt(name = "complex")]
    Complex,
}

#[derive(StructOpt)]
#[structopt(
    name = "hex 2 bin translator",
    about = "A tool to translate hex dumps back into binaries"
)]
pub struct Args {
    #[structopt(short = "i", long = "input", help = "Input hex file")]
    pub in_file: PathBuf,
    #[structopt(short = "o", long = "output", help = "Output bin file")]
    pub out_file: PathBuf,
    #[structopt(short = "c", long = "chunk", help = "Size of reading chunks")]
    pub chunk_size: usize,
    #[structopt(
        short = "t",
        long = "threads",
        help = "Number of threads",
        default_value = "8"
    )]
    pub thread_count: usize,
    #[structopt(subcommand)]
    pub mode: Mode,
}

impl Args {
    pub fn init() -> Self {
        Self::from_args()
    }
}
