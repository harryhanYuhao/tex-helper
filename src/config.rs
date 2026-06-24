//! Config struct for configuration
//! The default config file is $HOME/.config/tex-helper/config.toml

use std::error::Error;
use toml;

use crate::cli::Cli;
use crate::utils;
use serde::{Deserialize, Serialize};

// This struct is passed to configure the behaviour of this crate
// This config can be read from a toml config file.
//
// We use Option<String> deliberately.
// Since the String itself can be empty, in other language we assume that an empty strings gives a
// `None` val. We do not follow this paradime, instead we use rust's build in option and superior option struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    main_file_name: Option<String>,
    latex_binary: Option<String>,
    debug: DebugLevel,

    // This field stores the log info. As config is initialised before logger, those info can only
    // be logged after the logged is initialised
    log_warn_message: Vec<String>,
    log_debug_message: Vec<String>,
}

/// The debug level, which is the same as simplelog::LevelFilter
/// The default level is Warn
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum DebugLevel {
    Off,
    Error,
    #[default]
    Warn,
    Debug,
    Trace,
}

impl Config {
    fn default() -> Self {
        Config {
            main_file_name: Some("main.tex".into()),
            latex_binary: None,
            debug: DebugLevel::default(),
            log_warn_message: vec![],
            log_debug_message: vec![],
        }
    }

    /// Read config file, etc
    pub fn init(cli: &Cli) -> Self {
        let mut warn_msg = vec![];
        let mut config = Self::read_config_file().unwrap_or_else(|e| {
            warn_msg.push(format!("Failed to read config file: {}", e));
            Self::default()
        });

        if let Some(level) = cli.debug {
            match level {
                0 => config.debug = DebugLevel::Off,
                1 => config.debug = DebugLevel::Error,
                2 => config.debug = DebugLevel::Warn,
                3 => config.debug = DebugLevel::Debug,
                4 => config.debug = DebugLevel::Trace,
                _ => config.debug = DebugLevel::default(),
            }
        }

        let latex_binary = utils::which_latex_binary();
        if latex_binary.is_none() {
            warn_msg.push(format!("No Known Latex Binary Found!"));
        }
        config.log_warn_message.extend(warn_msg);
        config.latex_binary = latex_binary;
        config
    }

    pub fn get_main_file_name(&self) -> String {
        self.main_file_name
            .clone()
            .unwrap_or(Self::default().main_file_name.unwrap())
    }

    pub fn get_latex_binary(&self) -> Option<String> {
        self.latex_binary.clone()
    }

    pub fn get_debug_level(&self) -> DebugLevel {
        self.debug.clone()
    }

    fn read_config_file() -> Result<Self, Box<dyn Error>> {
        use std::env;
        use std::fs;
        let home_dir: String = env::var("HOME")?;
        let config_path = home_dir + "/.config/tex-helper/config.toml";

        let mut debug_msgs: String = String::new();
        let mut warn_msgs: String = String::new();
        let config_content = match fs::read_to_string(&config_path) {
            Ok(s) => {
                debug_msgs =
                    format!("Config file {} read successfully", &config_path);
                s
            }
            Err(e) => {
                warn_msgs = format!(
                    "Failed to read config file: {}. Using default config",
                    e
                );

                // return default config for now.
                // TODO: improve error handling
                return Ok(Self::default());
            }
        };
        let mut config: Config = toml::from_str(&config_content)?;
        config.log_warn_message.push(warn_msgs);
        config.log_debug_message.push(debug_msgs);
        Ok(config)
    }

    pub(crate) fn flush_log(&self) {
        if self.log_warn_message.len() != 0 {
            for i in &self.log_warn_message {
                warn!("{}", i);
            }
        }
        if self.log_debug_message.len() != 0 {
            for i in &self.log_debug_message {
                debug!("{}", i);
            }
        }
    }
}
