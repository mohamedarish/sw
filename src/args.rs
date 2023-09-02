use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to list (Default = ".")
    pub path: Option<PathBuf>,

    /// Display all files including hidden files
    #[arg(short, long)]
    pub all: bool,

    /// Display the output in list format
    #[arg(short, long)]
    pub list: bool,
}
