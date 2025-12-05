use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sb3Project {
    pub targets: Vec<Target>,
    pub monitors: Vec<Monitor>,
    pub extensions: Vec<String>,
    #[serde(rename = "extensionURLs")]
    pub extension_urls: HashMap<String, String>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub semver: String,
    pub vm: String,
    pub agent: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    #[serde(rename = "isStage")]
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<String, (String, Value)>, // ID -> [Name, Value]
    pub lists: HashMap<String, (String, Vec<Value>)>, // ID -> [Name, List]
    pub broadcasts: HashMap<String, String>,         // ID -> Name
    pub blocks: HashMap<String, Block>,
    pub comments: HashMap<String, Comment>,
    #[serde(rename = "currentCostume")]
    pub current_costume: i32,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub volume: f64,
    #[serde(rename = "layerOrder")]
    pub layer_order: i32,

    // Stage specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tempo: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "videoTransparency")]
    pub video_transparency: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "videoState")]
    pub video_state: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "textToSpeechLanguage"
    )]
    pub text_to_speech_language: Option<String>,

    // Sprite specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "rotationStyle")]
    pub rotation_style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Block {
    Normal(NormalBlock),
    TopLevelPrimitive(TopLevelPrimitive), // For variables/lists dropped on workspace? Rarely used in JSON but possible
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NormalBlock {
    pub opcode: String,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub inputs: HashMap<String, Input>,
    pub fields: HashMap<String, Field>,
    pub shadow: bool,
    #[serde(rename = "topLevel")]
    pub top_level: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation: Option<Mutation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopLevelPrimitive {
    // This variant is tricky because Scratch JSON is weird.
    // Usually blocks are objects.
    // We'll assume everything is a NormalBlock for now as compiled code usually is.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Input {
    // Input is [shadow_type, value] or [shadow_type, block_id] or [shadow_type, block_id, shadow_block_id]
    // But in JSON it can be complex.
    // Simpler representation: a Vec<Value>.
    // [1, [10, "10"]] -> Shadow (1), Number (10)
    // [2, "block_id"] -> No Shadow (2), pointing to block
    // [3, "block_id", [10, "10"]] -> Shadow obfuscated (3)
    Generic(Vec<Value>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Field {
    // [Value, ID] or [Value]
    Generic(Vec<Value>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mutation {
    #[serde(rename = "tagName")]
    pub tag_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Value>>,
    #[serde(rename = "proccode")]
    pub proccode: Option<String>,
    #[serde(rename = "argumentids")]
    pub argumentids: Option<String>, // JSON string array
    #[serde(rename = "argumentnames")]
    pub argumentnames: Option<String>, // JSON string array
    #[serde(rename = "argumentdefaults")]
    pub argumentdefaults: Option<String>, // JSON string array
    #[serde(rename = "warp")]
    pub warp: Option<String>, // "true" or "false" or boolean? Usually string in mutation
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<String>, // "1" or null
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub minimized: bool,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Costume {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub name: String,
    pub bitmap_resolution: Option<i32>,
    pub md5ext: String,
    #[serde(rename = "dataFormat")]
    pub data_format: String,
    #[serde(rename = "rotationCenterX")]
    pub rotation_center_x: f64,
    #[serde(rename = "rotationCenterY")]
    pub rotation_center_y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sound {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub name: String,
    pub md5ext: String,
    #[serde(rename = "dataFormat")]
    pub data_format: String,
    pub rate: Option<i32>,
    #[serde(rename = "sampleCount")]
    pub sample_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Monitor {
    pub id: String,
    pub mode: String,
    pub opcode: String,
    pub params: HashMap<String, String>,
    #[serde(rename = "spriteName")]
    pub sprite_name: Option<String>,
    pub value: Value,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub visible: bool,
    #[serde(rename = "sliderMin")]
    pub slider_min: f64,
    #[serde(rename = "sliderMax")]
    pub slider_max: f64,
    #[serde(rename = "isDiscrete")]
    pub is_discrete: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn test_comment_serialization() {
        let comment = Comment {
            block_id: Some("123".to_string()),
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            minimized: false,
            text: "hello".to_string(),
        };
        let json = to_string(&comment).unwrap();
        assert!(json.contains("\"blockId\":\"123\""));
    }
}
