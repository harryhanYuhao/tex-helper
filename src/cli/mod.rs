/// This file defines the behaviour of CLI. 
/// As the whole program is a CLI executable, this is the actual "main" file
use std::fs;
use std::path::PathBuf;

mod compile;
mod format;
mod init;

use crate::utils;
use crate::config;

use clap::{Parser, Subcommand};

use simplelog::{
    ColorChoice, CombinedLogger, Config as LogConfig, LevelFilter, TermLogger,
    TerminalMode,
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)] // read from cargo.toml
#[command(propagate_version = true)]
pub(crate) struct Cli {
    /// Show AST tree for development debugging
    #[arg(short, long, default_value_t = false)]
    pub(crate) debug: bool,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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

        #[arg(short, long, value_name = "outfile")]
        outfile: Option<String>,
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

/// CLI entry function
pub fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = config::Config::init(&cli);
    init_logger(config.debug()); // cli.debug is entered by the user flags.  
                            // This is a feature of clap crate.


    debug!("Config: {:?}", config);
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Init {
            doc_mode: doc_mod,
            package_name,
        } => {
            init::init_tex_project(package_name, doc_mod, &config)?;
            info!("Initialized LaTeX package `{package_name}` with document mode `{doc_mod}`");
        }
        Commands::Format {
            target,
            in_place,
            outfile,
        } => {
            let mut path = PathBuf::from(".");
            path.push(target);
            if !path.exists() {
                return Err(format!(
                    "Target path `{}` does not exist",
                    path.display()
                )
                .into());
            }
            let res = format::format(&path, &config)?;

            // TODO: debug level output
            if *in_place {
                // Backup original file
                fs::copy(target, &format!(".{}.backup", target))?;
                info!("Backed up original file to `.{}.backup`", target);
                utils::overwrite_to_file_path_buf(
                    &PathBuf::from(target),
                    &res,
                )?;
            }

            // write to outfile
            if let Some(out) = outfile {
                utils::overwrite_to_file_path_buf(&PathBuf::from(out), &res)?;
            }
        }
    }
    Ok(())
}
