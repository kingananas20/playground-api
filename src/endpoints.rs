mod execute;

pub use execute::{ExecuteRequest, ExecuteResponse};
use serde::{Deserialize, Serialize};

pub enum Endpoints {
    Execute,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Edition {
    #[serde(rename = "2024")]
    Edition2024,

    #[serde(rename = "2021")]
    Edition2021,

    #[serde(rename = "2018")]
    Edition2018,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CrateType {
    #[serde(rename = "bin")]
    Binary,

    #[serde(rename = "lib")]
    Library,
}
