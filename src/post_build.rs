//! Controls bundling of the plug-in after build. Utilizes the `cargo-post` crate. Requires settings to configured in `config.toml`.
//!
//! # Example
//! `Cargo.toml`
//! ```toml
//! [package.metadata.cargo-post.dependencies]
//! directories = "*"
//! toml = "*"
//! serde = { version = "1.0", features = ["derive"] }
//! fm_plugin = "*"
//! ```
//!
//! `config.toml`
//! ```toml
//! [filemaker]
//! ext_path = "/path/to/Extentions"
//! bin_path = "/Applications/FileMaker Pro.app"
//!
//! [plugin]
//! name = "plugin name"
//! ```
//!
//! `post_build.rs`
//! ```rust
//! #[cfg(any(target_os = "windows", target_os = "macos"))]
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     fm_plugin::post_build::bundle_plugin()?;
//!     Ok(())
//! }
//! ```

use directories::UserDirs;
use std::error::Error;
#[cfg(target_os = "macos")]
use std::fs::{create_dir_all, remove_dir_all};
use std::fs::{rename, File};
use std::path::Path;
use std::process;

use crate::config::{read_config, BuildError, Config};

/// Handles bundling, renaming, and moving of the lib after build.
/// 1. Loads prefs from `config.toml`
/// 2. Clears `plugin.log` on the desktop.
/// 3. Bundles the plug-in (on mac).
/// 4. Renames the plug-in.
/// 5. Moves plug-in to FileMaker Extensions folder.
/// 6. Launches FileMaker.
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
