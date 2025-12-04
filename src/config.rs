use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct ScrustConfig {
    pub project: ProjectConfig,
    pub stage: TargetConfig,
    pub sprite: Option<Vec<TargetConfig>>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ProjectConfig {
    pub name: String,
    pub output: PathBuf,
    pub extensions: Option<Vec<ExtensionConfig>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtensionConfig {
    Simple(String),
    Detailed(DetailedExtensionConfig),
}

#[derive(Deserialize, Debug, Clone)]
pub struct DetailedExtensionConfig {
    pub name: Option<String>,
    pub id: Option<String>,
    pub source: Option<String>,
    pub definition: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
pub struct TargetConfig {
    pub name: Option<String>,
    pub path: PathBuf,
}
