use super::{Channel, CrateType, Edition, Mode};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CompileRequest {
    pub target: CompileTarget,
    pub channel: Channel,
    pub mode: Mode,
    #[serde(default)]
    pub edition: Edition,
    #[serde(rename = "crateType")]
    pub crate_type: CrateType,
    pub tests: bool,
    #[serde(default)]
    pub backtrace: bool,
    pub code: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CompileResponse {
    pub success: bool,
    #[serde(rename = "exitDetail")]
    pub exit_detail: String,
    pub code: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AssemblyFlavor {
    Att,
    Intel,
}

#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DemangleAssembly {
    Demangle,
    Mangle,
}

#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessAssembly {
    Filter,
    Raw,
}

#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CompileTarget {
    Assembly(AssemblyFlavor, DemangleAssembly, ProcessAssembly),
    Hir,
    LlvmIr,
    Mir,
    Wasm,
}
