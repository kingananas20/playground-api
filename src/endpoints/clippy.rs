use super::{Channel, CrateType, Edition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClippyRequest {
    pub channel: Channel,
    #[serde(rename = "crateType")]
    pub crate_type: CrateType,
    pub edition: Edition,
    pub code: String,
}

impl ClippyRequest {
    pub fn new(
        channel: Channel,
        crate_type: CrateType,
        edition: Edition,
        code: String,
    ) -> ClippyRequest {
        ClippyRequest {
            channel,
            crate_type,
            edition,
            code,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClippyResponse {
    pub success: bool,
    pub exit_detail: String,
    pub stdout: String,
    pub stderr: String,
}
