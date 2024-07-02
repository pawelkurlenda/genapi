use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EntityDefinition {
    pub entity: String,
    #[serde(rename = "endpoints")]
    pub endpoint_types: Vec<EndpointType>,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum EndpointType {
    #[serde(rename = "GET_BY_ID")]
    GetById,
    #[serde(rename = "GET_LIST")]
    GetList,
    #[serde(rename = "CREATE")]
    Create,
    #[serde(rename = "UPDATE")]
    Update,
    #[serde(rename = "DELETE")]
    Delete
}

impl EntityDefinition {
    pub fn new(path: &Path) -> Self {
        let mut file = File::open(path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

        serde_json::from_str(&contents).expect("Failed to parse JSON")
    }
}