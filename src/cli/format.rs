//! Format function definedi nthis module is called by the cli module to perform formatting
//! This module essentially calls the latex_interpreter::formatter module  to format the AST
use std::error::Error;
use std::path::PathBuf;

use crate::config::Config;

use crate::latex_interpreter::{
    formatter::format as format_private, parser::parse, scanner::scan,
};
use crate::utils::*;

/// May PANIC!
/// This function is called by the cli module to format the files
pub fn format(file_path: &PathBuf, config: &Config) -> Result<String, Box<dyn Error>> {
    let file_input = FileInput::from_file_path(file_path)?;
    let tokens = scan(file_input.clone())?;
    let ast = match parse(&tokens, file_input) {
        Ok(res) => res,
        Err(e) => panic!("Internal parsing error: {}", e),
    };
    if config.is_debug(){
        debug!("AST: {}", ast.lock().unwrap());
    }

    let res: String = format_private(ast, config)?;

    Ok(res)
}
