use lazy_static::lazy_static;
extern crate simplelog;
#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};

mod cli;
mod config;
mod utils;
mod latex_interpreter;

use config::Config;

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
}

fn init() {
    use simplelog::{
        ColorChoice, CombinedLogger, Config as LogConfig, LevelFilter, TermLogger, TerminalMode,
    };

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        LogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let mut config = CONFIG.lock().unwrap();
    // init the config with some system settings
    // i.e., finding latex binary, etc
    // the configuration will be further updated by the cli
    // TODO: config init is likely redundant, remove it.
    config.init();
}

fn main() {
    init();
    match cli::cli() {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    }
}
