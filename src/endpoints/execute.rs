use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub channel: String,
    pub mode: String,
    #[serde(default)]
    pub edition: String,
    #[serde(rename = "crateType")]
    pub crate_type: String,
    pub tests: bool,
    #[serde(default)]
    pub backtrace: bool,
    pub code: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExecuteResponse {
    pub success: bool,
    #[serde(rename = "exitDetail")]
    pub exit_detail: String,
    pub stdout: String,
    pub stderr: String,
}
