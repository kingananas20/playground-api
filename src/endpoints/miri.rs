use super::Edition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MiriRequest {
    pub code: String,
    pub edition: Edition,
    pub tests: bool,
    #[serde(rename = "aliasingModel")]
    pub aliasing_model: Option<AliasingModel>,
}

impl MiriRequest {
    pub fn new(
        code: String,
        edition: Edition,
        tests: bool,
        aliasing_model: Option<AliasingModel>,
    ) -> Self {
        Self {
            code,
            edition,
            tests,
            aliasing_model,
        }
    }
}

impl Default for MiriRequest {
    fn default() -> Self {
        Self {
            code: "fn main() { println!(\"Hello, world!\"); }".to_owned(),
            edition: Edition::Edition2024,
            tests: false,
            aliasing_model: Some(AliasingModel::Stacked),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MiriResponse {
    pub success: bool,
    pub exit_detail: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AliasingModel {
    Stacked,
    Tree,
}
