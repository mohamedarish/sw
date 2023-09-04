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

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::args::Cli;
    use clap::{error::ErrorKind, Parser};

    #[test]
    fn test_parse_args_valid() {
        let args = vec!["myapp", "--all", "--list", "/path/to/some/folder"];
        let cli = Cli::parse_from(args.clone());
        assert_eq!(cli.path, Some(PathBuf::from("/path/to/some/folder")));
        assert!(cli.all);
        assert!(cli.list);
    }

    #[test]
    fn test_parse_args_missing_path() {
        let args = vec!["myapp", "--all", "--list"];
        let cli = Cli::parse_from(args.clone());
        assert_eq!(cli.path, None);
        assert!(cli.all);
        assert!(cli.list);
    }

    #[test]
    fn test_parse_args_no_options() {
        let args = vec!["myapp"];
        let cli = Cli::parse_from(args.clone());
        assert_eq!(cli.path, None);
        assert!(!cli.all);
        assert!(!cli.list);
    }

    #[test]
    fn test_parse_args_invalid_option() {
        let args = vec!["myapp", "--invalid-option"];
        let result = Cli::try_parse_from(args.clone());

        match result {
            Err(e) => assert_eq!(e.kind(), ErrorKind::UnknownArgument),
            _ => panic!("Expected an error for an invalid option"),
        }
    }
}
