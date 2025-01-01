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