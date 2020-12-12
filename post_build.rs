use directories::UserDirs;
use serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{create_dir_all, read_to_string, remove_dir_all, rename, File};
use std::path::Path;
use std::process;
use toml;

fn main() -> Result<(), Box<dyn Error>> {
    let config = read_config()?;
    kill_filemaker(&config)?;
    clear_log_file()?;
    bundle_plugin(&config)?;
    launch_filemaker(&config)?;
    Ok(())
}

fn clear_log_file() -> Result<(), Box<dyn Error>> {
    let user_dirs = UserDirs::new().ok_or(PostBuildError::LogFile)?;
    let dir = user_dirs.desktop_dir().ok_or(PostBuildError::LogFile)?;
    let path = Path::join(&dir, "plugin.log");
    let _ = File::create(path)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn kill_filemaker(config: &Config) {
    process::Command::new("taskkill")
        .arg("/IM")
        .arg("FileMaker Pro.exe")
        .arg("/F")
        .spawn()
        .ok();
}

#[cfg(target_os = "macos")]
fn kill_filemaker(config: &Config) -> Result<(), Box<dyn Error>> {
    let app_path = Path::new(&config.filemaker.bin_path);
    let app = app_path.file_stem().ok_or(PostBuildError::FileMaker)?;
    process::Command::new("pkill").arg(app).spawn().ok();
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
fn bundle_plugin(config: &Config) -> Result<(), Box<dyn Error>> {
    let manifest = env!("CRATE_MANIFEST_DIR");
    let name = env!("CARGO_PKG_NAME");
    let from = format!("{}/target/release/{}.dll", manifest, name);
    let to = format!("{}/Extensions/{}.fmx64", FILEMAKER_EXT_PATH, PLUGIN_NAME);
    rename(from, to);

    Ok(())
}

#[cfg(target_os = "macos")]
fn bundle_plugin(config: &Config) -> Result<(), Box<dyn Error>> {
    let out_dir = env!("CRATE_OUT_DIR");
    let package_name = Path::new(env!("CRATE_MANIFEST_DIR"))
        .file_name()
        .ok_or(PostBuildError::Bundle)?
        .to_string_lossy()
        .replace("-", "_");
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

#[derive(Debug)]
enum PostBuildError {
    LogFile,
    FileMaker,
    Bundle,
}

impl Display for PostBuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for PostBuildError {}

#[derive(Deserialize, Debug)]
struct Config {
    filemaker: FileMaker,
    plugin: Plugin,
}

#[derive(Deserialize, Debug)]
struct FileMaker {
    ext_path: String,
    bin_path: String,
}

#[derive(Deserialize, Debug)]
struct Plugin {
    name: String,
}

fn read_config() -> Result<Config, Box<dyn Error>> {
    let config_path = Path::new(env!("CRATE_MANIFEST_DIR"));
    let config_path = config_path.join("config.toml");
    let contents = read_to_string(config_path)?;

    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
