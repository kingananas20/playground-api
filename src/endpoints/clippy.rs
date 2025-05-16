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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClippyResponse {
    pub success: bool,
    #[serde(rename = "exitDetail")]
    pub exit_detail: String,
}
