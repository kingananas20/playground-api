mod compile;
mod execute;

pub use execute::{ExecuteRequest, ExecuteResponse};
use serde::{Deserialize, Serialize};

pub(crate) enum Endpoints {
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

    #[serde(rename = "2015")]
    Edition2015,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CrateType {
    #[serde(rename = "bin")]
    Binary,

    #[serde(rename = "lib")]
    Library,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Mode {
    #[serde(rename = "debug")]
    Debug,

    #[serde(rename = "release")]
    Release,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Channel {
    #[serde(rename = "stable")]
    Stable,

    #[serde(rename = "beta")]
    Beta,

    #[serde(rename = "nightly")]
    Nightly,
}
