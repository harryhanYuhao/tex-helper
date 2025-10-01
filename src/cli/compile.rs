use colored::Colorize;
use std::env;
use std::error::Error;
use std::fs;
use std::process::Command;

use crate::CONFIG;

pub fn compile(main_file_path: &str) -> Result<(), Box<dyn Error>> {
    let config = CONFIG.lock().unwrap();

    let binding = config.get_latex_binary();
    let binary = match &binding {
        Some(b) => b,
        None => {
            return Err(format!(
                "{}: {}",
                "Latex Binary".red(),
                "Not Found".red()
            )
            .into());
        }
    };

    fs::create_dir_all(".build/")?;
    fs::copy(main_file_path, ".build/main.tex")?;

    env::set_current_dir(".build/")?;

    let output = Command::new(&binary)
        .arg(main_file_path)
        .arg("--pdf")
        .output()?;

    fs::copy("main.pdf", "../main.pdf")?;
    env::set_current_dir("..")?;

    if output.status.success() {
        println!(
            "{}",
            format!(
                "{}: {}",
                "Success".green(),
                "Compilation Successful".green()
            )
        );
    } else {
        println!(
            "{}",
            format!(
                "{} \n{}: {}",
                String::from_utf8(output.stdout)?,
                "Error".red(),
                "Compilation Failed".red()
            )
        );
    }
    Ok(())
}
