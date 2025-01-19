mod compile;
mod init;

use clap::{Args, Parser, Subcommand, ValueEnum};

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
        package_name: String,
        #[arg(
            long,
            require_equals = true,
            value_name = "DOC_MODE",
            default_value_t = DocMode::Article,
            value_enum
            )
        ]
        doc_mode: DocMode,
    },
    /// Compile the latex files
    Compile { targets: Vec<String> },
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
        Commands::Init {
            doc_mode: init_args,
            package_name,
        } => match init_args {
            DocMode::Report => {
                init::init_report(package_name)?;
            }
            DocMode::Book => {
                init::init_book(package_name)?;
            }
            DocMode::Article => {
                init::init_article(package_name)?;
            }
            DocMode::Letter => {
                init::init_letter(package_name)?;
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
