use directories::UserDirs;
use std::error::Error;
#[cfg(target_os = "macos")]
use std::fs::{create_dir_all, remove_dir_all};
use std::fs::{rename, File};
use std::path::Path;
use std::process;

use crate::config::{read_config, BuildError, Config};

pub fn bundle_plugin() -> Result<(), Box<dyn Error>> {
    let config_path =
        Path::new(option_env!("CRATE_MANIFEST_DIR").ok_or("CRATE_MANIFEST_DIR not set")?);
    let config = read_config(config_path)?;
    clear_log_file()?;
    bundle_plugin_command(&config)?;
    launch_filemaker(&config)?;
    Ok(())
}

fn clear_log_file() -> Result<(), Box<dyn Error>> {
    let user_dirs = UserDirs::new().ok_or(BuildError::LogFile)?;
    let dir = user_dirs.desktop_dir().ok_or(BuildError::LogFile)?;
    let path = Path::join(&dir, "plugin.log");
    let _ = File::create(path)?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn launch_filemaker(config: &Config) -> Result<(), Box<dyn Error>> {
    process::Command::new("open")
        .arg(&config.filemaker.bin_path)
        .spawn()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn launch_filemaker(config: &Config) -> Result<(), Box<dyn Error>> {
    process::Command::new(&config.filemaker.bin_path).spawn()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn bundle_plugin_command(config: &Config) -> Result<(), Box<dyn Error>> {
    let out_dir = option_env!("CRATE_OUT_DIR").ok_or("CRATE_OUT_DIR not set")?;
    let package_name = get_package_name()?;
    let from = format!("{}\\{}.dll", out_dir, package_name);
    let to = format!("{}/{}.fmx64", config.filemaker.ext_path, config.plugin.name);
    rename(from, to)?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn bundle_plugin_command(config: &Config) -> Result<(), Box<dyn Error>> {
    let out_dir = env!("CRATE_OUT_DIR");
    let package_name = get_package_name()?;
    let plugin_path = format!(
        "{}/{}.fmplugin",
        config.filemaker.ext_path, config.plugin.name
    );

    remove_dir_all(&plugin_path).ok();
    create_dir_all(&plugin_path)?;

    let bin_path = format!("{}/Contents/MacOS", plugin_path);
    create_dir_all(&bin_path)?;

    let from = format!("{}/lib{}.dylib", out_dir, package_name);
    let to = format!("{}/{}", bin_path, config.plugin.name);
    rename(from, to)?;

    Ok(())
}

fn get_package_name() -> Result<String, Box<dyn Error>> {
    Ok(
        Path::new(option_env!("CRATE_MANIFEST_DIR").ok_or("CRATE_MANIFEST_DIR not set")?)
            .file_name()
            .ok_or(BuildError::Bundle)?
            .to_string_lossy()
            .replace("-", "_"),
    )
}
