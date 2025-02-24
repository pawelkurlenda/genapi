use std::fs::File;
use std::io::Read;
use std::path::Path;
use actix_web::web::ServiceConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct EntityDefinition {
    pub entity: String,
    #[serde(rename = "endpoints")]
    pub endpoint_types: Vec<EndpointType>,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FieldDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
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

    /*pub fn get_routes(&self) -> ConfigRoute {

    }*/
}

pub struct ConfigRoute {
    pub scope: String,
    pub routes: Vec<String>
}

impl EndpointType {
    pub fn to_str(&self) -> String {
        match self {
            EndpointType::GetById => "GET_BY_ID".to_string(),
            EndpointType::GetList => "GET_LIST".to_string(),
            EndpointType::Create => "CREATE".to_string(),
            EndpointType::Update => "UPDATE".to_string(),
            EndpointType::Delete => "DELETE".to_string()
        }
    }
}

pub trait Configurable {
    fn configure(cfg: &mut ServiceConfig);
}