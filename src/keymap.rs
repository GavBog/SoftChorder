use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct KeymapCatagory {
    name: String,
    description: String,
    icon: Option<String>,
    display: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
    actions: HashMap<u16, Keymap>,
}

#[derive(Deserialize, Debug)]
pub struct Keymap {
    id: Option<String>,
    title: Option<String>,
    icon: Option<String>,
    display: Option<String>,
    description: Option<String>,
    variant: Option<String>,
    #[serde(rename = "variantOf")]
    variant_of: Option<u16>,
    #[serde(rename = "keyCode")]
    key_code: Option<String>,
}

/// Parse Keymaps into HashMap
pub async fn parse_keymaps(dir: &str) -> HashMap<u16, Option<String>> {
    let mut keymaps = HashMap::new();
    let mut files = tokio::fs::read_dir(dir).await.unwrap();
    loop {
        let file = files.next_entry().await.unwrap();
        if let Some(file) = file {
            let file = file.path();
            if let Some(keymap) = parse_yaml_file(file.to_str().unwrap()).await {
                for (key, value) in keymap.actions {
                    keymaps.insert(key, value.id);
                }
            }
        } else {
            break;
        }
    }
    keymaps
}

/// Parse Yaml File into KeymapCatagory
async fn parse_yaml_file(file: &str) -> Option<KeymapCatagory> {
    let file = tokio::fs::read_to_string(file).await.unwrap();
    if let Ok(keymap) = serde_yaml::from_str(&file) {
        return Some(keymap);
    }
    None
}
