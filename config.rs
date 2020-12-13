use serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::Path;

#[allow(dead_code)]
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
    let contents = read_to_string(config_path)?;

    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
