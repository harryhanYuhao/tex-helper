use crate::utils;

use crate::CONFIG;

#[derive(Debug)]
pub struct Config {
    main_file_name: String,
    latex_binary: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            main_file_name: "main.tex".into(),
            latex_binary: None,
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
}

pub fn get_main_file_name() -> String {
    let config = CONFIG.lock().unwrap();
    config.get_main_file_name()
}

pub fn get_latex_binary() -> Option<String> {
    let config = CONFIG.lock().unwrap();
    config.get_latex_binary()
}
