use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub path: Option<PathBuf>,

    #[arg(short, long)]
    pub all: bool,

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
        let cli = Cli::parse_from(&[""]);

        assert_eq!(cli.path, None);
        assert_eq!(cli.all, false);
        assert_eq!(cli.list, false);
    }

    #[test]
    fn test_parse_args() {
        let args = ["", "--all"];
        let cli: Cli = Cli::parse_from(&args);

        assert_eq!(cli.all, true);
        assert_eq!(cli.list, false);
    }

    #[test]
    fn test_parse_path() {
        let args = ["", "path/to/some/file"];
        let cli: Cli = Cli::parse_from(&args);

        assert_eq!(cli.path, Some(PathBuf::from("path/to/some/file")));
        assert_eq!(cli.all, false);
        assert_eq!(cli.list, false);
    }

    #[test]
    fn test_parse_list() {
        let args = ["", "--list"];
        let cli: Cli = Cli::parse_from(&args);

        assert_eq!(cli.list, true);
        assert_eq!(cli.all, false);
    }
}
