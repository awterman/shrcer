use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub modules: Vec<Module>,
    #[serde(default)]
    pub exports: Exports,
    #[serde(default)]
    pub path: Path,
    #[serde(default)]
    pub aliases: HashMap<String, AliasConfig>,
    #[serde(default)]
    pub znap: Option<ZnapConfig>,
    #[serde(default)]
    pub tools: HashMap<String, ToolConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Module {
    #[serde(default)]
    pub zsh_code: String,
    #[serde(default)]
    pub fish_code: String,
    #[serde(default)]
    pub plugins: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Exports {
    #[serde(flatten)]
    pub common: HashMap<String, String>,
    #[serde(default)]
    pub zsh: HashMap<String, String>,
    #[serde(default)]
    pub fish: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Path {
    #[serde(default)]
    pub prepend: Vec<String>,
    #[serde(default)]
    pub append: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AliasConfig {
    pub cmd: String,
    #[serde(default)]
    pub if_condition: Option<String>,
    #[serde(rename = "if")]
    #[serde(default)]
    pub if_tool: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ZnapConfig {
    pub custom_path: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ToolConfig {
    #[serde(default)]
    pub if_cmd: Option<String>,
    #[serde(default)]
    pub if_file: Option<String>,
    #[serde(default)]
    pub zsh_code: String,
    #[serde(default)]
    pub fish_code: String,
    #[serde(default)]
    pub exports: HashMap<String, String>,
    #[serde(default)]
    pub path: Path,
}

pub fn parse_config(content: &str) -> Result<Config> {
    let config: Config = toml::from_str(content)?;
    Ok(config)
} 