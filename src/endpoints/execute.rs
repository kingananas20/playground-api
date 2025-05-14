use super::{Channel, CrateType, Edition, Mode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct ExecuteRequest {
    pub channel: Channel,
    pub mode: Mode,
    pub edition: Edition,
    #[serde(rename = "crateType")]
    pub crate_type: CrateType,
    pub tests: bool,
    #[serde(default)]
    pub backtrace: bool,
    pub code: String,
}

impl ExecuteRequest {
    #[allow(dead_code)]
    fn new(
        channel: Channel,
        mode: Mode,
        edition: Edition,
        crate_type: CrateType,
        tests: bool,
        backtrace: bool,
        code: String,
    ) -> ExecuteRequest {
        ExecuteRequest {
            channel,
            mode,
            edition,
            crate_type,
            tests,
            backtrace,
            code,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteResponse {
    pub success: bool,
    #[serde(rename = "exitDetail")]
    pub exit_detail: String,
    pub stdout: String,
    pub stderr: String,
}
