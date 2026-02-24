use lazy_static::lazy_static;
extern crate simplelog;
#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};

mod cli;
mod config;
mod latex_interpreter;
mod markdown_interpreter;
mod utils;

fn main() {
    match cli::cli() {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    }
}
