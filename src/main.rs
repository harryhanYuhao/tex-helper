use lazy_static::lazy_static;
use colored::Colorize;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};

mod cli;
mod config;
mod utils;

use config::Config;

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
}

fn init() {
    pretty_env_logger::init();
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
            error!("{}: {}", "Error".red(), format!("{}", e));
        }
    }
}
