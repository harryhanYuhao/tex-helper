use std::path::PathBuf;

mod compile;
mod format;
mod init;

use clap::{Parser, Subcommand};

use simplelog::{
    ColorChoice, CombinedLogger, Config as LogConfig, LevelFilter, TermLogger,
    TerminalMode,
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)] // read from cargo.toml
#[command(propagate_version = true)]
struct Cli {
    /// More detailed logs
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Creating a new latex package with <PACKAG_NAME>
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
    /// Format Latex
    Format {
        target: String,

        #[arg(short, long, default_value_t = false)]
        in_place: bool,
    }, // Compile the latex files
       // Compile { targets: Vec<String> },
}

/// Init logger according to debug flag
fn init_logger(debug: bool) {
    let log_filter = if debug {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };

    CombinedLogger::init(vec![TermLogger::new(
        log_filter,
        LogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}

pub fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    init_logger(cli.debug);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Init {
            doc_mode: doc_mod,
            package_name,
        } => {
            init::init_tex_project(package_name, doc_mod)?;
            info!("Initialized LaTeX package `{package_name}` with document mode `{doc_mod}`");
        }
        Commands::Format { target, in_place } => {
            let mut path = PathBuf::from(".");
            path.push(target);
            if !path.exists() {
                return Err(format!(
                    "Target path `{}` does not exist",
                    path.display()
                )
                .into());
            }
            format::format(&path)?;
        }
    }
    Ok(())
}
