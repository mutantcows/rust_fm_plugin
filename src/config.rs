//! Handles the force quitting of FileMaker before building the plug-in.
//! This serves two purposes:
//! 1. Allowing you to re-load the plug-in in FileMaker.
//! 2. Sometimes FileMaker locks a `.dll` in the target folder during testing and prevents cargo from building.
//!
//! # Example
//! `Cargo.toml`
//! ```toml
//! [build-dependencies]
//! fm_plugin = "*"
//! ```
//! `config.toml`
//! ```toml
//![filemaker]
//!ext_path = "/path/to/Extentions"
//!bin_path = "/Applications/FileMaker Pro.app"
//!kill = true
//!launch = true
//!
//![plugin]
//!name = "plugin name"
//!bundle = true
//!move_to_ext = true
//!
//![code_signing]
//!sign = true
//!signtool_path = "/path/to/signtool.exe"
//!cert_path = "/path/to/cert.p12"
//!cert_pass = "password"
//!timestamp_url = "http://cert.timestamp.server.com"
//!
//![log]
//!path = "/path/to/plugin.log"
//!clear_on_launch = true
//! ```
//! `build.rs`
//! ```rust
//! #[cfg(any(target_os = "windows", target_os = "macos"))]
//!fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    fm_plugin::kill_filemaker()?;
//!    Ok(())
//!}
//!#  #[cfg(target_os = "linux")]
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!#     Ok(())
//!# }
//! ```

use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::{
    fmt::{Display, Formatter},
    path::Path,
    process,
};

#[derive(Debug)]
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(crate) enum BuildError {
    FileMaker,
    Bundle,
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl Error for BuildError {}

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) filemaker: FileMaker,
    pub(crate) plugin: Plugin,
    pub(crate) log: Log,
    pub(crate) code_signing: CodeSigning,
}

#[derive(Deserialize, Debug)]
pub(crate) struct FileMaker {
    pub(crate) ext_path: Option<String>,
    pub(crate) bin_path: Option<String>,
    pub(crate) launch: bool,
    pub(crate) kill: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Plugin {
    pub(crate) name: String,
    pub(crate) bundle: bool,
    pub(crate) move_to_ext: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct CodeSigning {
    pub(crate) sign: bool,
    pub(crate) signtool_path: String,
    pub(crate) cert_path: String,
    pub(crate) cert_pass: String,
    pub(crate) timestamp_url: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Log {
    pub(crate) path: Option<String>,
    pub(crate) clear_on_launch: bool,
}

pub(crate) fn read_config() -> Result<Config, Box<dyn Error>> {
    let current_dir = env::current_dir().unwrap();
    let mut config_path = current_dir.join("config.toml");

    // if the config isn't in the current dir, check all folders in the current dir
    if !config_path.is_file() {
        for entry in current_dir.read_dir()? {
            if let Ok(f) = entry {
                if f.file_type()?.is_dir() {
                    let test_path = current_dir.join(f.file_name()).join("config.toml");
                    if test_path.is_file() {
                        config_path = test_path;
                        break;
                    }
                }
            }
        }
    }
    let contents = read_to_string(&config_path)
        .map_err(|e| format!("config read failed. path: {:?}, error: {}", config_path, e))?;

    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

/// Force quits FileMaker using the path provided in `config.toml`.
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn kill_filemaker() -> Result<(), Box<dyn Error>> {
    if let Ok(profile) = env::var("PROFILE") {
        if profile == "release" {
            let config = read_config()?;
            kill_filemaker_command(&config)?;
        }
    }
    Ok(())
}

/// Force quits FileMaker using the path provided in `config.toml`.
#[cfg(target_os = "linux")]
pub fn kill_filemaker() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn kill_filemaker_command(config: &Config) -> Result<(), Box<dyn Error>> {
    if !config.filemaker.kill {
        return Ok(());
    }
    let app_path = Path::new(config.filemaker.bin_path.as_ref().unwrap());
    let app = app_path.file_name().ok_or(BuildError::FileMaker)?;
    process::Command::new("taskkill")
        .arg("/IM")
        .arg(app)
        .arg("/F")
        .spawn()?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn kill_filemaker_command(config: &Config) -> Result<(), Box<dyn Error>> {
    if !config.filemaker.kill {
        return Ok(());
    }
    let app_path = Path::new(config.filemaker.bin_path.as_ref().unwrap());
    let app = app_path.file_stem().ok_or(BuildError::FileMaker)?;
    process::Command::new("pkill").arg(app).spawn().ok();
    Ok(())
}
