use super::{CrateType, Edition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct ExecuteRequest {
    pub channel: String,
    pub mode: String,
    pub edition: Edition,
    #[serde(rename = "crateType")]
    pub crate_type: CrateType,
    pub tests: bool,
    #[serde(default)]
    pub backtrace: bool,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteResponse {
    pub success: bool,
    #[serde(rename = "exitDetail")]
    pub exit_detail: String,
    pub stdout: String,
    pub stderr: String,
}
