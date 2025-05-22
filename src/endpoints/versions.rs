use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct VersionsResponse {
    pub stable: ChannelVersion,
    pub beta: ChannelVersion,
    pub nightly: ChannelVersion,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChannelVersion {
    pub rustc: Version,
    pub rustfmt: Version,
    pub clippy: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miri: Option<Version>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Version {
    pub version: String,
    pub hash: String,
    pub date: String,
}
