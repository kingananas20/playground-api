//! Module which contains request and response types for the various endpoints

mod clippy;
mod compile;
mod crates;
mod execute;
mod format;
mod gist;
mod macro_expansion;
mod miri;
mod versions;

pub use clippy::{ClippyRequest, ClippyResponse};
pub use compile::{
    AssemblyFlavor, CompileRequest, CompileResponse, CompileTarget, DemangleAssembly,
    ProcessAssembly,
};
pub use crates::{CrateInformation, CratesResponse};
pub use execute::{ExecuteRequest, ExecuteResponse};
pub use format::{FormatRequest, FormatResponse};
pub use gist::{GistCreateRequest, GistResponse};
pub use macro_expansion::{MacroExpansionRequest, MacroExpansionResponse};
pub use miri::{AliasingModel, MiriRequest, MiriResponse};
pub use versions::{ChannelVersion, Version, VersionsResponse};

use serde::{Deserialize, Serialize};

pub(crate) enum Endpoints {
    Execute,
    Compile,
    Format,
    Clippy,
    Miri,
    MacroExpansion,
    Crates,
    Versions,
    GistCreate,
    GistGet(String),
}

/// Represents the Rust edition to use.
///
/// Each edition introduces new language features and idioms while maintaining compatibility.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "poise-bot", derive(poise::ChoiceParameter))]
pub enum Edition {
    /// Rust 2024 Edition
    #[cfg_attr(feature = "poise-bot", name = "Edition 2024")]
    #[serde(rename = "2024")]
    Edition2024,

    /// Rust 2021 Edition
    #[cfg_attr(feature = "poise-bot", name = "Edition 2021")]
    #[serde(rename = "2021")]
    Edition2021,

    /// Rust 2018 Edition
    #[cfg_attr(feature = "poise-bot", name = "Edition 2018")]
    #[serde(rename = "2018")]
    Edition2018,

    /// Rust 2015 Edition
    #[cfg_attr(feature = "poise-bot", name = "Edition 2015")]
    #[serde(rename = "2015")]
    Edition2015,
}

/// Defines the type of crate to be compiled: binary or library.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "poise-bot", derive(poise::ChoiceParameter))]
pub enum CrateType {
    /// A binary crate with a `main` function, compiled to an executable.
    #[serde(rename = "bin")]
    Binary,

    /// A library crate, with a specified library type.
    #[serde(rename = "lib")]
    Library,
}

/// Indicates whether to compile code in debug or release mode.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "poise-bot", derive(poise::ChoiceParameter))]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Compile with debug information and no optimizations.
    Debug,

    /// Compile with optimizations for performance.
    Release,
}

/// Specifies the Rust release channel to use.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "poise-bot", derive(poise::ChoiceParameter))]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    /// Stable channel – tested and officially released features.
    Stable,

    /// Beta channel – pre-release testing for the next stable release.
    Beta,

    /// Nightly channel – bleeding-edge features and experimental tools.
    Nightly,
}
