use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::Path;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::process;

#[derive(Debug)]
pub enum BuildError {
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
pub struct Config {
    pub filemaker: FileMaker,
    pub plugin: Plugin,
}

#[derive(Deserialize, Debug)]
pub struct FileMaker {
    pub ext_path: String,
    pub bin_path: String,
}

#[derive(Deserialize, Debug)]
pub struct Plugin {
    pub name: String,
}

pub fn read_config(config_path: &Path) -> Result<Config, Box<dyn Error>> {
    let config_path = config_path.join("config.toml");
    let contents = read_to_string(config_path)?;

    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn kill_filemaker() -> Result<(), Box<dyn Error>> {
    let manifest = env!("CARGO_MANIFEST_DIR");

    if env::var("PROFILE").unwrap() == "release" {
        let config = read_config(Path::new(manifest)).unwrap();
        kill_filemaker_command(&config)?;
    }

    if cfg!(target_os = "windows") {
        println!(r"cargo:rustc-link-search={}/libraries/Win/x64", manifest);
    } else if cfg!(target_os = "macos") {
        println!(
            r"cargo:rustc-link-search=framework={}/libraries/Mac",
            manifest
        );
    } else if cfg!(target_os = "linux") {
        println!(r"cargo:rustc-link-search={}/libraries/Linux", manifest);
    }

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
