use lazy_static::lazy_static;
extern crate simplelog;
#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};

mod cli;
mod config;
mod utils;
mod latex_interpreter;
mod markdown_interpreter;

use config::Config;

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
}

fn init() {
    let mut config = CONFIG.lock().unwrap();
    // init the config with some system settings
    // i.e., finding latex binary, etc
    // the configuration will be further updated by the cli
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
