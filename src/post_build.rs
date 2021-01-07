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
//! kill = true
//! launch = true
//!
//! [plugin]
//! name = "plugin name"
//! bundle = true
//! move_to_ext = true
//!
//! [code_signing]
//! sign = false
//! signtool_path = "/path/to/signtool.exe"
//! cert_path = "/path/to/cert.p12"
//! cert_pass = "password"
//! timestamp_url = "http://cert.timestamp.server.com"
//!
//! [log]
//! path = "/path/to/plugin.log"
//! clear_on_launch = true
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

use std::error::Error;
use std::fs::File;
#[cfg(target_os = "macos")]
use std::fs::{create_dir_all, remove_dir_all};
use std::path::Path;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::{fs::rename, path::PathBuf, process};

use crate::config::{read_config, BuildError, Config};

/// Handles bundling, renaming, and moving of the lib after build.
/// 1. Loads prefs from `config.toml`
/// 2. Clears the log file.
/// 3. Bundles the plug-in (on mac).
/// 4. Renames the plug-in.
/// 5. Moves plug-in to FileMaker Extensions folder.
/// 6. Signs the plug-in.
/// 7. Launches FileMaker.
pub fn bundle_plugin() -> Result<(), Box<dyn Error>> {
    let config = read_config()?;
    clear_log_file(&config)?;
    bundle_plugin_command(&config)?;
    launch_filemaker(&config)?;
    Ok(())
}

fn clear_log_file(config: &Config) -> Result<(), Box<dyn Error>> {
    if !config.log.clear_on_launch || config.log.path.is_none() {
        return Ok(());
    }
    let path = Path::new(&config.log.path.as_ref().unwrap()).to_path_buf();
    let _ = File::create(path)?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn launch_filemaker(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.filemaker.bin_path.is_some() && config.filemaker.launch {
        process::Command::new("open")
            .arg(&config.filemaker.bin_path)
            .spawn()?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn launch_filemaker(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.filemaker.bin_path.is_some() && config.filemaker.launch {
        process::Command::new(config.filemaker.bin_path.as_ref().unwrap()).spawn()?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn bundle_plugin_command(config: &Config) -> Result<(), Box<dyn Error>> {
    if !config.plugin.bundle {
        return Ok(());
    }
    let mut out_dir = option_env!("CRATE_OUT_DIR");
    if out_dir.is_none() {
        out_dir = option_env!("CARGO_MANIFEST_DIR");
    }
    let out_dir = out_dir.unwrap();

    let mut package_name = get_package_name()?;
    package_name.push_str(".dll");
    let from = Path::new(out_dir).join(package_name);

    let mut plugin_name = config.plugin.name.to_owned();
    plugin_name.push_str(".fmx64");

    let to: PathBuf;
    if config.filemaker.ext_path.is_some() && config.plugin.move_to_ext {
        to = Path::new(config.filemaker.ext_path.as_ref().unwrap()).join(plugin_name);
    } else {
        to = Path::new(&out_dir).join(plugin_name);
    }
    rename(from, &to)?;

    if config.code_signing.sign {
        sign_code_command(&to, &config)?;
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn bundle_plugin_command(config: &Config) -> Result<(), Box<dyn Error>> {
    if !config.plugin.bundle {
        return Ok(());
    }
    let out_dir = option_env!("CRATE_OUT_DIR").ok_or("CRATE_OUT_DIR not set")?;
    let package_name = get_package_name()?;
    let mut plugin_name = config.plugin.name.to_owned();
    plugin_name.push_str(".fmplugin");
    let plugin_path: PathBuf;

    if config.plugin.move_to_ext && config.filemaker.ext_path.is_some() {
        plugin_path = Path::new(config.filemaker.ext_path.as_ref().unwrap()).join(plugin_name);
        remove_dir_all(&plugin_path).ok();
        create_dir_all(&plugin_path)?;
    } else {
        plugin_path = Path::new(out_dir).join(plugin_name);
        remove_dir_all(&plugin_path).ok();
        create_dir_all(&plugin_path)?;
    }

    let bin_path = Path::new(&plugin_path).join("/Contents/MacOS");
    create_dir_all(&bin_path)?;

    let mut lib_name = String::from("lib");
    lib_name.push_str(&package_name);
    let from = Path::new(out_dir).join(lib_name);

    let to = Path::new(&bin_path).join(&config.plugin.name);
    rename(from, to)?;

    Ok(())
}

fn get_package_name() -> Result<String, Box<dyn Error>> {
    Ok(
        Path::new(option_env!("CRATE_MANIFEST_DIR").unwrap_or(env!("CARGO_MANIFEST_DIR")))
            .file_name()
            .ok_or(BuildError::Bundle)?
            .to_string_lossy()
            .replace("-", "_"),
    )
}

#[cfg(target_os = "windows")]
fn sign_code_command(plugin_path: &Path, config: &Config) -> Result<(), Box<dyn Error>> {
    if !config.code_signing.sign {
        return Ok(());
    }
    let signtool_path = Path::new(&config.code_signing.signtool_path);
    process::Command::new(signtool_path)
        .arg("sign")
        .arg("/tr")
        .arg(&config.code_signing.timestamp_url)
        .arg("/f")
        .arg(&config.code_signing.cert_path)
        .arg("/p")
        .arg(&config.code_signing.cert_pass)
        .arg(plugin_path)
        .output()?;
    Ok(())
}

#[cfg(target_family = "unix")]
fn sign_code_command(plugin_path: &Path, config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(target_os = "linux")]
fn bundle_plugin_command(_config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(target_os = "linux")]
fn launch_filemaker(_config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
