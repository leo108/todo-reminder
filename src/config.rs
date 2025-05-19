use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>,
    pub parameters: Parameters,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub paths: Vec<PathBuf>,
    pub language: String,
    pub file_extensions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Parameters {
    pub editor_url: Option<String>,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
