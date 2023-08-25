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
mod tests {
    use std::path::PathBuf;

    use clap::Parser;

    use crate::Cli;

    #[test]
    fn test_default_values() {
        let cli = Cli::parse_from([""]);

        assert_eq!(cli.path, None);
        assert!(!cli.all);
        assert!(!cli.list);
    }

    #[test]
    fn test_parse_args() {
        let args = ["", "--all"];
        let cli: Cli = Cli::parse_from(args);

        assert!(cli.all);
        assert!(!cli.list);
    }

    #[test]
    fn test_parse_path() {
        let args = ["", "path/to/some/file"];
        let cli: Cli = Cli::parse_from(args);

        assert_eq!(cli.path, Some(PathBuf::from("path/to/some/file")));
        assert!(!cli.all);
        assert!(!cli.list);
    }

    #[test]
    fn test_parse_list() {
        let args = ["", "--list"];
        let cli: Cli = Cli::parse_from(args);

        assert!(cli.list);
        assert!(cli.all);
    }
}
