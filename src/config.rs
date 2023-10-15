use derive_more::Display;
use directories::ProjectDirs;
use serde_derive::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::{fs, io};
use toml::de;

#[derive(Deserialize)]
pub struct Config {
    pub data_dir: PathBuf,
}

#[derive(Debug, Display)]
pub enum ConfigError {
    #[display(fmt = "failed to read config ({})", _0)]
    IOError(io::Error),
    #[display(fmt = "failed to get config directory path")]
    NoPath,
    #[display(fmt = "failed to deserialize config ({})", _0)]
    DeserializationError(de::Error),
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::DeserializationError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::IOError(err)
    }
}

impl From<de::Error> for ConfigError {
    fn from(err: de::Error) -> Self {
        ConfigError::DeserializationError(err)
    }
}

impl Config {
    pub fn read() -> Result<Self, ConfigError> {
        let project_dirs =
            ProjectDirs::from("se", "Nivalis", "RequiM").ok_or(ConfigError::NoPath)?;
        let config_file_path = project_dirs.config_dir().join("Config.toml");

        if config_file_path.exists() {
            let document = fs::read_to_string(config_file_path)?;
            Ok(de::from_str(&document)?)
        } else {
            Ok(Self::default(&project_dirs))
        }
    }

    fn default(project_dirs: &ProjectDirs) -> Self {
        Self {
            data_dir: project_dirs.data_local_dir().to_path_buf(),
        }
    }
}
