use crate::config::ExtensionConfig;
use anyhow::{Context, Result};
use base64::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct Extension {
    #[allow(dead_code)]
    pub name: String,
    pub id: String,
    #[serde(default)]
    pub blocks: HashMap<String, BlockDef>,
    #[serde(skip)]
    pub project_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    Command,
    Reporter,
    Hat,
    #[serde(rename = "c_block", alias = "c_shape")]
    CShape,
    Boolean,
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Command
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlockDef {
    pub opcode: String,
    #[serde(default)]
    pub block_type: BlockType,
    #[serde(default)]
    pub inputs: HashMap<String, InputMapping>,
    #[serde(default)]
    pub fields: HashMap<String, FieldMapping>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum InputMapping {
    Arg { arg: usize },
    // We can add more types later if needed, e.g., specific values
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum FieldMapping {
    Value { value: String },
}

pub fn load_extensions(
    extensions_dir: &Path,
    configs: &Option<Vec<ExtensionConfig>>,
    config_base_dir: &Path,
) -> Result<Vec<Extension>> {
    let mut extensions = Vec::new();

    // If no configs provided, load all from directory (legacy behavior, assuming implicit usage)

    // Helper to load from file
    let load_file = |path: &Path| -> Result<Extension> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read extension definition from {:?}", path))?;
        let mut ext: Extension = toml::from_str(&content)
            .with_context(|| format!("Failed to parse extension definition from {:?}", path))?;
        ext.project_id = Some(ext.id.clone());
        Ok(ext)
    };

    let mut dir_extensions: HashMap<String, Extension> = HashMap::new();
    if extensions_dir.exists() {
        for entry in fs::read_dir(extensions_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "toml") {
                if let Ok(ext) = load_file(&path) {
                    dir_extensions.insert(ext.id.clone(), ext);
                }
            }
        }
    }

    if let Some(configs) = configs {
        for config in configs {
            match config {
                ExtensionConfig::Simple(id) => {
                    if id == "return" {
                        // Special built-in "return" extension
                        extensions.push(Extension {
                            name: "Return".to_string(),
                            id: "return".to_string(),
                            blocks: HashMap::new(),
                            project_id: None,
                        });
                        continue;
                    }

                    if let Some(mut ext) = dir_extensions.get(id).cloned() {
                        ext.project_id = Some(id.clone());
                        extensions.push(ext);
                    } else {
                        println!(
                            "Warning: Extension '{}' not found in extensions directory.",
                            id
                        );
                    }
                }
                ExtensionConfig::Detailed(detailed) => {
                    let mut ext = if let Some(def_path) = &detailed.definition {
                        let full_path = if def_path.is_absolute() {
                            def_path.clone()
                        } else {
                            config_base_dir.join(def_path)
                        };
                        load_file(&full_path)?
                    } else {
                        // Look up by name or id in scanned extensions
                        let key = detailed.name.as_ref().or(detailed.id.as_ref());
                        if let Some(k) = key {
                            if let Some(e) = dir_extensions.get(k).cloned() {
                                e
                            } else {
                                anyhow::bail!("Extension '{}' definition not found.", k);
                            }
                        } else {
                            anyhow::bail!(
                                "Extension config must provide definition path or name/id."
                            );
                        }
                    };

                    // Determine project_id
                    if let Some(source) = &detailed.source {
                        if source.starts_with("http://")
                            || source.starts_with("https://")
                            || source.starts_with("data:")
                        {
                            ext.project_id = Some(source.clone());
                        } else {
                            // Handle local file
                            let source_path = if Path::new(source).is_absolute() {
                                PathBuf::from(source)
                            } else {
                                config_base_dir.join(source)
                            };

                            let content = fs::read(&source_path).with_context(|| {
                                format!("Failed to read extension source file {:?}", source_path)
                            })?;
                            let encoded = BASE64_STANDARD.encode(&content);
                            let data_uri =
                                format!("data:application/javascript;base64,{}", encoded);
                            ext.project_id = Some(data_uri);
                        }
                    } else if let Some(id) = &detailed.id {
                        ext.project_id = Some(id.clone());
                    } else if let Some(name) = &detailed.name {
                        ext.project_id = Some(name.clone());
                    }

                    extensions.push(ext);
                }
            }
        }
    } else {
        for (_, ext) in dir_extensions {
            extensions.push(ext);
        }
    }

    Ok(extensions)
}
