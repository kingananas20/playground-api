mod clippy;
mod compile;
mod execute;
mod format;

pub use clippy::{ClippyRequest, ClippyResponse};
pub use compile::{
    AssemblyFlavor, CompileRequest, CompileResponse, CompileTarget, DemangleAssembly,
    ProcessAssembly,
};
pub use execute::{ExecuteRequest, ExecuteResponse};
pub use format::{FormatRequest, FormatResponse};

use serde::{Deserialize, Serialize};

pub(crate) enum Endpoints {
    Execute,
    Compile,
    Format,
    Clippy,
}

/// Represents the Rust edition to use.
///
/// Each edition introduces new language features and idioms while maintaining compatibility.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Edition {
    /// Rust 2024 Edition
    #[serde(rename = "2024")]
    Edition2024,

    /// Rust 2021 Edition
    #[serde(rename = "2021")]
    Edition2021,

    /// Rust 2018 Edition
    #[serde(rename = "2018")]
    Edition2018,

    /// Rust 2015 Edition
    #[serde(rename = "2015")]
    Edition2015,
}

/// Defines the type of crate to be compiled: binary or library.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CrateType {
    /// A binary crate with a `main` function, compiled to an executable.
    #[serde(rename = "bin")]
    Binary,

    /// A library crate, with a specified library type.
    #[serde(rename = "lib")]
    Library,
}

/// Specifies the type of Rust library to build.
///
/// These map to different kinds of compiled library outputs.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum LibraryType {
    /// A standard Rust library (`lib.rs`).
    Lib,

    /// A dynamic system library (`.so`, `.dylib`, etc.).
    Dylib,

    /// A Rust-specific compiled library format.
    Rlib,

    /// A statically linked library (`.a`, etc.).
    Staticlib,

    /// A C-compatible dynamic library.
    Cdylib,

    /// A procedural macro crate.
    ProcMacro,
}

/// Indicates whether to compile code in debug or release mode.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Compile with debug information and no optimizations.
    Debug,

    /// Compile with optimizations for performance.
    Release,
}

/// Specifies the Rust release channel to use.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    /// Stable channel – tested and officially released features.
    Stable,

    /// Beta channel – pre-release testing for the next stable release.
    Beta,

    /// Nightly channel – bleeding-edge features and experimental tools.
    Nightly,
}
