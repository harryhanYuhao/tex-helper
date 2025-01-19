mod compile;
mod init;

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialised the current directory for latex. Require args: report, book, article, or
    /// letter
    Init {
        #[arg(default_value_t = DocMode::Article)]
        init_args: DocMode,
    },
    /// Compile the latex files 
    Compile {
        targets: Vec<String>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DocMode {
    Report,
    Book,
    Article,
    Letter,
}

impl std::fmt::Display for DocMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocMode::Report => write!(f, "report"),
            DocMode::Book => write!(f, "book"),
            DocMode::Article => write!(f, "article"),
            DocMode::Letter => write!(f, "letter"),
        }
    }
}

pub fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Init { init_args } => match init_args {
            DocMode::Report => {
                init::init_report()?;
            }
            DocMode::Book => {
                init::init_book()?;
            }
            DocMode::Article => {
                init::init_article()?;
            }
            DocMode::Letter => {
                init::init_letter()?;
            }
        },
        Commands::Compile { targets } => {
            for i in targets {
                compile::compile(i)?;
            }
        }
    }
    Ok(())
}
