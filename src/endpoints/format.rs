use super::{Channel, CrateType, Edition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormatRequest {
    pub channel: Channel,
    pub crate_type: CrateType,
    pub edition: Edition,
    pub code: String,
}

impl FormatRequest {
    pub fn new(
        channel: Channel,
        crate_type: CrateType,
        edition: Edition,
        code: String,
    ) -> FormatRequest {
        FormatRequest {
            channel,
            crate_type,
            edition,
            code,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormatResponse {
    pub success: bool,
    #[serde(rename = "exitDetail")]
    pub exit_detail: String,
    pub code: String,
}
