use crate::{Error, Reportable};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;


const TOML_FILE: &str = "floki.toml";
const MAIN_SERVICE: &str = "app";
const CLIENT_SERVICE: &str = "client";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub floki: Floki,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Floki {
    pub main_service: Option<String>,
    pub client_service: Option<String>
}

impl Config {
    /// read from path or default to 'floki.toml'
    pub fn read(path: &Option<String>) -> Result<Self, Reportable> {
        let path = path.as_deref().unwrap_or(TOML_FILE);
        Config::try_read(path).map_err(|e| e.file_context("read config", path))
    }
    fn try_read(path: &str) -> Result<Self, Error> {
        log::debug!("Reading config file {path}");
        let toml = fs::read_to_string(path)?;
        log::trace!("Config file content:\n{toml}");
        Ok(toml::from_str(&toml)?)
    }
    pub fn projects(&self) -> Projects {
        Projects {
            app: param_or_folder(&self.floki.main_service, MAIN_SERVICE),
            client: param_or_folder(&self.floki.client_service, CLIENT_SERVICE),
        }
    }
    pub fn save_default_file() -> Result<(), Reportable> {
        Self::try_save_default().map_err(|e| e.file_context("", TOML_FILE))
    }

    fn try_save_default() -> Result<(), Error> {
        log::debug!("Adding default floki.toml file");
        let path = format!("../{}", TOML_FILE);
        let toml = std::fs::read_to_string(path)?;
        log::trace!("Content of floki.toml:\n{toml}");
        Ok(std::fs::write(TOML_FILE, toml.as_bytes())?)
    }
}

#[derive(Debug, Default)]
pub struct Projects {
    pub app: Option<String>,
    pub client: Option<String>,
}

fn param_or_folder(param: &Option<String>, folder: &str) -> Option<String> {
    if let Some(path) = param {
        Some(path.to_string())
    } else {
        let path = PathBuf::from(folder);
        if path.exists() && path.is_dir() {
            Some(folder.to_string())
        } else {
            None
        }
    }
}
