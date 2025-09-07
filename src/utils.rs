// use std::error::Error;
use std::process::Command;

fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn which_latex_binary() -> Option<String> {
    let candidates = vec!["latexmk", "pdflatex"];
    for i in candidates {
        if command_exists(i) {
            return Some(i.to_string());
        }
    }

    None
}

pub(crate) fn legal_characters_for_dir_name(instr: &str) -> Vec<char> {
    let illegal_c = ['/', '\\'];
    instr.chars().filter(|&c| illegal_c.contains(&c)).collect()
}

// This function assumes the file does not exist or is empty 
// create the file if it does not exist
// wipes the file and writes content to it if it does
pub(crate) fn overwrite_to_file(path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test{
}
