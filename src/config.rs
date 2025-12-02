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
}

#[derive(Deserialize, Debug)]
pub struct TargetConfig {
    pub name: Option<String>,
    pub path: PathBuf,
}
