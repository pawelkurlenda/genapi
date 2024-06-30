use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EntityDefinition {
    pub entity: String,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

impl EntityDefinition {
    pub fn new(path: &Path) -> Self {
        let mut file = File::open(path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

        serde_json::from_str(&contents).expect("Failed to parse JSON")
    }
}