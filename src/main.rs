#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_default,
    clippy::unwrap_used
)]

use std::{error, fs, io, result};

use clap::Parser;
use sw::{Cli, Directory};

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handler = stdout.lock();

    let width: usize;

    if let Some((w, _)) = term_size::dimensions() {
        width = w;
    } else {
        return Err(Error::from("Cannot get the size of the terminal"));
    }

    let args = Cli::parse();

    let directory = if let Some(dirpath) = args.path {
        if dirpath.is_file() {
            return Err(Error::from("Given argument is a dir and not a path"));
        }

        fs::read_dir(dirpath).expect("Cannot read the ")
    } else {
        fs::read_dir(".").expect("Cannot read the file")
    };

    let directory_content = Directory::from(directory, args.all, args.list);

    directory_content.print_nlist(&mut handler, width, args.all);

    Ok(())
}
