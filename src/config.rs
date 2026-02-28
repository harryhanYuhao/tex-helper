//! This crate creates a config struct for configuration of the command line tool.
 
use std::error::Error;
use toml;

use crate::utils;
use crate::cli::Cli;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DebugLevel{
    None,
    Info,
    Full
}

// This struct is passed to configure the behaviour of this crate 
// This config can be read from a toml config file. 
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    main_file_name: Option<String>,
    latex_binary: Option<String>,
    debug: Option<bool>,
}

impl Config {
    fn default() -> Self {
        Config {
            main_file_name: Some("main.tex".into()),
            latex_binary: None,
            debug: Some(false),
        }
    }


    /// Read config file, etc
    pub fn init(cli: &Cli) -> Self{
        let mut config = Self::get_config_from_file().unwrap_or_else(|e| {
            warn!("Failed to read config file: {}", e);
            Self::default()
        });
        if cli.debug {
            config.debug = Some(true);
        }
        let latex_binary = utils::which_latex_binary();
        if latex_binary.is_none() {
            warn!("No Known Latex Binary Found!")
        }
        config.latex_binary = latex_binary;
        config
    }

    pub fn get_main_file_name(&self) -> String {
        self.main_file_name.clone().unwrap_or(Self::default().main_file_name.unwrap())
    }

    pub fn get_latex_binary(&self) -> Option<String> {
        self.latex_binary.clone()
    }

    pub fn debug(&self) -> bool {
        self.debug.clone().unwrap_or(Self::default().debug.unwrap())
    }

    fn get_config_from_file() -> Result<Self, Box<dyn Error>>{
        use std::fs;
        use std::env;
        let home_dir: String = env::var("HOME")?;
        let config_str = match fs::read_to_string(home_dir + "/.config/tex-helper/config.toml"){
            Ok(s) => {
                debug!("Config file read successfully");
                s
            },
            Err(e) => {
                // warn!("Failed to read config file: {}", e);
                // return default config for now. 
                // TODO: Better error handling 
                return Ok(Self::default());
            }
        };
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}

