#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_default,
    clippy::unwrap_used
)]

use std::{env, error, io, result};

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

        Directory::from(dirpath, args.all, args.list)
    } else {
        Directory::from(
            env::current_dir().expect("Cannot access current dir"),
            args.all,
            args.list,
        )
    };

    directory.print_nlist(&mut handler, width, args.all);

    Ok(())
}
