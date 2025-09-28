//! This file contains the `init` command logic
//! This file is not the initialisation of the crate

mod default_assets;

use crate::config;
use crate::utils;
use std::fs;
// use crate::utils::{self, illegal_characters_for_dir_name};

use std::error::Error;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

fn create_new_dir(package_name: &str) -> Result<(), Box<dyn Error>> {
    if Path::new(&package_name).exists() {
        return Err(format!(
            "{} already exists. Use a different package name.",
            package_name
        )
        .into());
    }

    create_dir_all(package_name)?;

    Ok(())
}

fn create_file_in_project_dir(
    package_name: &str,
    file_name: &str,
    content: &str,
) -> Result<(), Box<dyn Error>> {
    let file_path = PathBuf::from(package_name).join(file_name);
    utils::overwrite_to_file_path_buf(&file_path, content)?;

    debug!("Created {}", file_path.display());

    Ok(())
}

pub(super) fn init_tex_project(package_name: &str, doc_mode: &str) -> Result<(), Box<dyn Error>> {
    create_new_dir(package_name)?;

    create_file_in_project_dir(package_name, ".gitignore", &default_assets::gitignore())?;
    create_file_in_project_dir(
        package_name,
        "references.bib",
        &default_assets::reference_bib(),
    )?;

    create_single_file_preamble_content(package_name, doc_mode)?;

    Ok(())
}

fn create_single_file_preamble_content(
    package_name: &str,
    doc_mode: &str,
) -> Result<(), Box<dyn Error>> {
    let main_file_path = utils::get_main_file_path(package_name);

    let custom_file_path = custom_template_exists(doc_mode)?;

    if custom_file_path.is_empty() {
        // no custom template, create defaults
        create_main_with_defaults(package_name, doc_mode)?;
    } else {
        // use custom template
        if Path::new(&custom_file_path).is_dir() {
            info!("Using custom directory template at {custom_file_path}");
            utils::copy_dir_all(&custom_file_path, package_name)?;
        } else {
            info!("Using custom file template at {custom_file_path}");
            let content = fs::read_to_string(&custom_file_path)?;
            utils::overwrite_to_file_path_buf(&main_file_path, &content)?;
        }
    }

    Ok(())
}

fn create_main_with_defaults(package_name: &str, doc_mode: &str) -> Result<(), Box<dyn Error>> {
    let main_file_name = config::get_main_file_name();

    let ret = default_assets::default_preable(doc_mode);
    if ret.is_empty() {
        info!("Document mode {doc_mode} not recognized, using article as default.");
        create_file_in_project_dir(
            package_name,
            &main_file_name,
            &default_assets::default_preable("article"),
        )?;
    } else {
        create_file_in_project_dir(package_name, &main_file_name, &ret)?;
    }
    Ok(())
}

// This function is the default_asset_
fn create_structured_preamble_content(
    package_name: &str,
    doc_mode: &str,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

/// check if custom template exists in config dir
fn custom_template_exists(template_name: &str) -> Result<String, Box<dyn Error>> {
    let fp = format!("{}/{}", utils::get_config_dir()?, template_name);
    let fp_tex = format!("{}.tex", &fp);

    if fs::exists(&fp_tex)? {
        return Ok(fp_tex);
    } else if fs::exists(&fp)? {
        return Ok(fp);
    }
    Ok(String::new())
}

pub(super) fn get_single_page_preamble(doc_mode: &str) -> Result<String, Box<dyn Error>> {
    let custom_file_path = custom_template_exists(doc_mode)?;
    if custom_file_path.is_empty() {
        let ret = default_assets::default_preable(doc_mode);
        if ret.is_empty() {
            info!("Document mode {doc_mode} not recognized, using article as default.");
            return Ok(default_assets::default_preable("article"));
        }
        return Ok(default_assets::default_preable(doc_mode));
    }
    info!("Using custom {doc_mode} template at {custom_file_path}");
    return Ok(fs::read_to_string(custom_file_path)?);
}
