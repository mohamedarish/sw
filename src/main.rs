#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_default,
    clippy::unwrap_used
)]

pub mod args;
pub mod dir;
pub mod support;

use std::{env, error, fs, io, result};

use args::Cli;
use clap::Parser;
use dir::Directory;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handler = stdout.lock();

    let width;

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

        let path = match fs::canonicalize(dirpath) {
            Ok(a) => a,
            Err(e) => return Err(Error::from(e.to_string())),
        };

        Directory::from(&path, args.all, args.list)
    } else {
        let Ok(current_dir) = env::current_dir() else {
            return Err(Error::from("Cannot read current directory ."));
        };

        Directory::from(&current_dir, args.all, args.list)
    };

    match directory {
        Ok(d) => {
            let Ok(()) = d.display_output(&mut handler, width, args.all, args.list) else {
                return Err(Error::from("Cannot print the output in stdout"));
            };
        }
        Err(e) => {
            return Err(Error::from(e.to_string()));
        }
    }

    Ok(())
}
