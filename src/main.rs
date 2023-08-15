use std::{
    error, fs,
    io::{self, Write},
    result,
};

use clap::Parser;
use colored::Colorize;
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

    let directory_content = Directory::from(directory, args.all);

    if !directory_content.hidden_folders.is_empty() || !directory_content.folders.is_empty() {
        let mut count = 0;
        if args.all {
            write!(handler, "{: <25}", "..".bright_cyan().bold()).expect("Cannot write to stdout");
            write!(handler, "{: <25}", ".".bright_cyan().bold()).expect("Cannot write to stdout");
            count += 50;

            for file in directory_content.hidden_folders {
                if width - count < 40 {
                    writeln!(handler, "").expect("Cannot write to stdout");
                    count = 0;
                }

                write!(handler, "{: <25}", file.bright_cyan().bold())
                    .expect("Cannot write to stdout");

                count += 25;
            }
        }

        for file in directory_content.folders {
            if width - count < 40 {
                writeln!(handler, "").expect("Cannot write to stdout");
                count = 0;
            }

            write!(handler, "{: <25}", file.green().bold()).expect("Cannot write to stdout");

            count += 25;
        }

        writeln!(handler, "").expect("Cannot write to stdout");
    }

    if !directory_content.hidden_files.is_empty() || !directory_content.files.is_empty() {
        let mut count = 0;
        if args.all {
            for file in directory_content.hidden_files {
                if width - count < 40 {
                    writeln!(handler, "").expect("Cannot write to stdout");
                    count = 0;
                }

                write!(handler, "{: <25}", file.bright_cyan()).expect("Cannot write to stdout");

                count += 25;

                if width - count < 40 {
                    writeln!(handler, "").expect("Cannot write to stdout");
                    count = 0;
                }
            }
        }

        for file in directory_content.files {
            if width - count < 40 {
                writeln!(handler, "").expect("Cannot write to stdout");
                count = 0;
            }

            write!(handler, "{: <25}", file).expect("Cannot write to stdout");

            count += 25;
        }

        writeln!(handler, "").expect("Cannot write to stdout");
    }

    Ok(())
}
