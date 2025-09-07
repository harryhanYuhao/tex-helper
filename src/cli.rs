mod compile;
mod init;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]  // read from cargo.toml
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Creating a new latex package at <PACKAG_NAME>
    Init {
        package_name: String,
        #[arg(
            long,
            require_equals = true,
            value_name = "DOC_MODE",
            default_value_t = String::from("article"),
            )
        ]
        doc_mode: String,
    },
    /// Compile the latex files
    Compile { targets: Vec<String> },
}

pub fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Init {
            doc_mode: init_args,
            package_name,
        } => {
            init::init_tex_project(package_name, init_args)?
        },
        Commands::Compile { targets } => {
            for i in targets {
                compile::compile(i)?;
            }
        }
    }
    Ok(())
}
