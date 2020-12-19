use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::Path;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::process;

#[derive(Debug)]
pub(crate) enum BuildError {
    LogFile,
    FileMaker,
    Bundle,
}

impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for BuildError {}

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) filemaker: FileMaker,
    pub(crate) plugin: Plugin,
}

#[derive(Deserialize, Debug)]
pub(crate) struct FileMaker {
    pub(crate) ext_path: String,
    pub(crate) bin_path: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Plugin {
    pub(crate) name: String,
}

pub(crate) fn read_config(config_path: &Path) -> Result<Config, Box<dyn Error>> {
    let config_path = config_path.join("config.toml");
    let contents = read_to_string(&config_path)?;

    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn kill_filemaker(manifest_dir: &str) -> Result<(), Box<dyn Error>> {
    if env::var("PROFILE").unwrap() == "release" {
        let config = read_config(Path::new(manifest_dir))?;
        kill_filemaker_command(&config)?;
    }

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn kill_filemaker(manifest_dir: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn kill_filemaker_command(config: &Config) -> Result<(), Box<dyn Error>> {
    let app_path = Path::new(&config.filemaker.bin_path);
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
    let app_path = Path::new(&config.filemaker.bin_path);
    let app = app_path.file_stem().ok_or(BuildError::FileMaker)?;
    process::Command::new("pkill").arg(app).spawn().ok();
    Ok(())
}
