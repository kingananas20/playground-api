use serde::{Deserialize, Serialize};

/// Represents metadata about a crate available on the Rust playground.
///
/// Includes the crate's name, version, and unique identifier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrateInformation {
    /// The name of the crate.
    pub name: String,
    /// The version of the crate.
    pub version: String,
    /// The unique identifier for the crate.
    pub id: String,
}

/// A response containing a list of available crates on the Rust playground.
///
/// Each crate is represented by a [`CrateInformation`] struct.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CratesResponse {
    /// A list of available crates.
    pub crates: Vec<CrateInformation>,
}
