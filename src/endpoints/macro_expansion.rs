use super::Edition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MacroExpansionRequest {
    pub code: String,
    pub edition: Edition,
}

impl MacroExpansionRequest {
    pub fn new(code: String, edition: Edition) -> Self {
        Self { code, edition }
    }
}

impl Default for MacroExpansionRequest {
    fn default() -> Self {
        Self {
            code: "fn main() { println!(\"Hello, world!\"); }".to_owned(),
            edition: Edition::Edition2024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MacroExpansionResponse {
    pub success: bool,
    pub exit_detail: String,
    pub stdout: String,
    pub stderr: String,
}
