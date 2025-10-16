//! Format function definedi nthis module is called by the cli module to perform formatting
//! This module essentially calls the latex_interpreter::formatter module  to format the AST
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::config;
use crate::latex_interpreter::{
    formatter::format as format_private, parser::parse, scanner::scan,
};
use crate::utils::*;

/// May PANIC!
/// This function is called by the cli module to format the files
pub fn format(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file_input = FileInput::from_file_path(file_path)?;
    let tokens = scan(file_input.clone())?;
    let ast = match parse(&tokens, file_input) {
        Ok(res) => res,
        Err(e) => panic!("Parsing error: {}", e),
    };


    let res = format_private(ast);

    let output_path = format!("{}.formatted.tex", file_path.display());
    fs::write(&output_path, res.to_string())?;

    Ok(())
}
