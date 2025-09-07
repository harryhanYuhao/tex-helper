// This file contains the `init` command logic
// this file is not the initialisation of the crate

mod default_assets;

use crate::CONFIG;

use crate::utils::legal_characters_for_dir_name;
use colored::Colorize;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use crate::utils;

// auxiliary function: shall only be called wihthin the crate with simple logics
// TODO: Add support for windows
fn file_path_from_dir_and_filename(directory: &str, filename: &str) -> String {
    if directory.len() == 0 {
        panic!("file_path_string() called with directory string empty!");
    }
    if filename.len() == 0 {
        panic!("file_path_string() called with filename string empty!");
    }
    let last_char = directory.chars().last().unwrap();
    if last_char == '/' {
        return format!("{}{}", directory, filename);
    }
    format!("{}/{}", directory, filename)
}


// return Ok(file_path), file_path is the path for the main file
fn create_dir_and_main(package_name: &str) -> Result<String, Box<dyn Error>> {
    if legal_characters_for_dir_name(package_name).len() != 0 {
        return Err(format!(
            "{} is an illegal name for directory as it contains {:?}",
            package_name,
            legal_characters_for_dir_name(package_name)
        )
        .into());
    }

    if Path::new(&package_name).exists() {
        return Err(format!("{} already exists. Use a different package name.", package_name).into());
    }

    create_dir_all(package_name)?;

    let config = CONFIG.lock().unwrap();
    let main_file_name = config.get_main_file_name();
    let main_file_path = format!("{}/{}", package_name, main_file_name);

    info!("Created {main_file_path}");
    Ok(main_file_path)
}

fn create_gitignore(package_name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = file_path_from_dir_and_filename(package_name, ".gitignore");
    utils::overwrite_to_file(
        &file_path,
        &default_assets::default_gitignore(),
    )?;

    info!("Created {file_path}");
    Ok(())
}

fn create_reference_template(package_name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = file_path_from_dir_and_filename(package_name, "references.bib");
    utils::overwrite_to_file(
        &file_path,
        &default_assets::default_reference_bib()
    )?;

    info!("Created {file_path}");
    Ok(())
}

pub(super) fn init_tex_project(package_name: &str, doc_mode: &str) -> Result<(), Box<dyn Error>> {
    let main_file_path = create_dir_and_main(package_name)?;
    create_gitignore(package_name)?; 
    create_reference_template(package_name)?;
    match doc_mode {
        "article" => {
            utils::overwrite_to_file(
                &main_file_path,
                &default_assets::default_main_article()
            )?;
            info!("Created {main_file_path}");
        }
        "book" => {
            utils::overwrite_to_file(
                &main_file_path,
                &default_assets::default_main_book()
            )?;
            info!("Created {main_file_path}");
        }
        "letter" => {
            utils::overwrite_to_file(
                &main_file_path,
                &default_assets::default_main_letter()
            )?;
            info!("Created {main_file_path}");
        }
        "report" => {
            utils::overwrite_to_file(
                &main_file_path,
                &default_assets::default_main_report()
            )?;
            info!("Created {main_file_path}");
        }
        _ => {
            warn!("Unknown doc_mode {}, falling back to report", doc_mode);
        }
    }



    Ok(())
}
