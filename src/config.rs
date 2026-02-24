//! This crate creates a config struct for configuration of the command line tool.
use crate::utils;

#[derive(Debug)]
pub enum DebugLevel{
    None,
    Info,
    Full
}

#[derive(Debug)]
pub struct Config {
    main_file_name: String,
    latex_binary: Option<String>,
    debug: bool,
}

impl Config {
    pub fn new(debug: bool) -> Self {
        Config {
            main_file_name: "main.tex".into(),
            latex_binary: None,
            debug,
        }
    }

    pub fn init(&mut self) {
        let latex_binary = utils::which_latex_binary();
        if latex_binary.is_none() {
            warn!("No Known Latex Binary Found!")
        }
        self.latex_binary = latex_binary;
    }

    pub fn get_main_file_name(&self) -> String {
        self.main_file_name.clone()
    }

    pub fn get_latex_binary(&self) -> Option<String> {
        self.latex_binary.clone()
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }

}

