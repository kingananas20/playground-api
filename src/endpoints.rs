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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Edition {
    #[serde(rename = "2024")]
    Edition2024,

    #[serde(rename = "2021")]
    Edition2021,

    #[serde(rename = "2018")]
    Edition2018,

    #[serde(rename = "2015")]
    Edition2015,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CrateType {
    #[serde(rename = "bin")]
    Binary,

    #[serde(rename = "lib")]
    Library(LibraryType),
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LibraryType {
    Lib,
    Dylib,
    Rlib,
    Staticlib,
    Cdylib,
    ProcMacro,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Debug,
    Release,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
}
