use crate::{
    keymap::parse_keymaps,
    utils::{chord_to_string, sort_string},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct CharaCordFile {
    #[serde(rename = "charaVersion")]
    chara_version: u8,
    #[serde(rename = "type")]
    type_: String,
    history: Vec<Vec<CharaChordHistory>>,
}

#[derive(Deserialize, Debug)]
pub struct CharaChordHistory {
    #[serde(rename = "charaVersion")]
    chara_version: u8,
    #[serde(rename = "type")]
    type_: String,
    chords: Option<Vec<(Vec<u16>, Vec<u16>)>>,
    device: Option<String>,
    layout: Option<Vec<Vec<u16>>>,
    settings: Option<Vec<Option<u32>>>,
}

/// Parse Coords into HashMap
pub async fn parse_coords(file: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let config = parse_json_file(file).await;
    let chords = config.history[0][0].chords.as_ref().unwrap();
    let keymaps = parse_keymaps("keymaps").await;

    for (actions, phrases) in chords {
        let mut actions = chord_to_string(actions.clone(), &keymaps)
            .await
            // TODO: Change this Later
            .replace("KEY_", "")
            .to_lowercase();
        sort_string(&mut actions);
        let phrases = chord_to_string(phrases.clone(), &keymaps).await;
        result.insert(actions, phrases);
    }
    result
}

/// Parse Json File into CharaCordFile
async fn parse_json_file(file: &str) -> CharaCordFile {
    let file = tokio::fs::read_to_string(file).await.unwrap();
    serde_json::from_str(&file).unwrap()
}
