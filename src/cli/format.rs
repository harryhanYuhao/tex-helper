use std::error::Error;
use std::path::PathBuf;

use crate::latex_interpreter::{parser::parse, scanner::scan};
use crate::utils::*;
use crate::config;

/// May panic
/// This function is called by the cli module to format the files
pub fn format(target: &PathBuf) {

}

fn format_file(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file_input = FileInput::from_file_path(file_path)?;
    let tokens = scan(file_input)?;
    let ast = parse(&tokens)?;

    Ok(())
}
