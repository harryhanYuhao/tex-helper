use crate::config;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, io};

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

// This function assumes the file does not exist or is empty
// create the file if it does not exist
// wipes the file and writes content to it if it does
pub(crate) fn overwrite_to_file_path_buf(
    path: &PathBuf,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// TODO: add support for Windows
pub(crate) fn get_config_dir() -> Result<String, Box<dyn Error>> {
    use std::env;
    let home_dir = env::var("HOME")?;
    Ok(format!("{}/.config/tex-helper", home_dir))
}

pub(crate) fn get_main_file_path(package_name: &str) -> PathBuf {
    let main_file_name = config::get_main_file_name();
    PathBuf::from(package_name).join(main_file_name)
}

pub fn copy_dir_all(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            let created = dst.as_ref().join(entry.file_name());
            debug!("Creating file: {}", created.display());
            fs::copy(entry.path(), &created)?;
        }
    }
    Ok(())
}

/// Struct for IO and error handling
pub struct FileInput {
    pub file_path: PathBuf,
    pub content: String,
}

impl FileInput {
    pub fn from_file_path(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(&file_path)?;
        let file_path = PathBuf::from(file_path);
        Ok(FileInput { file_path, content })
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

#[cfg(test)]
mod test {}
