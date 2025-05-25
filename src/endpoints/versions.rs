use serde::{Deserialize, Serialize};

/// A response containing Rust compiler toolchain versions for different release channels.
///
/// Includes versions for stable, beta, and nightly channels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VersionsResponse {
    /// The stable channel versions.
    pub stable: ChannelVersion,
    /// The beta channel versions.
    pub beta: ChannelVersion,
    /// The nightly channel versions.
    pub nightly: ChannelVersion,
}

/// Tool versions for a specific Rust release channel.
///
/// Contains versions for rustc, rustfmt, clippy, and optionally miri.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChannelVersion {
    /// Version information for `rustc`.
    pub rustc: Version,
    /// Version information for `rustfmt`.
    pub rustfmt: Version,
    /// Version information for `clippy`.
    pub clippy: Version,
    /// Optional version information for `miri`, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miri: Option<Version>,
}

/// Version metadata for a specific tool in the Rust toolchain.
///
/// Includes the version string, commit hash, and release date.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Version {
    /// The version string (e.g., "1.70.0").
    pub version: String,
    /// The git commit hash of the release.
    pub hash: String,
    /// The release date (ISO 8601 format).
    pub date: String,
}
