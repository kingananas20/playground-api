use super::{Channel, CrateType, Edition, Mode};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct CompileRequest {
    pub target: String,
    #[serde(rename = "assemblyFlavor")]
    pub assembly_flavor: Option<String>,
    #[serde(rename = "demangleAssembly")]
    pub demangle_assembly: Option<String>,
    #[serde(rename = "processAssembly")]
    pub process_assembly: Option<String>,
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
#[derive(Debug, Clone, Deserialize)]
pub struct CompileResponse {
    pub success: bool,
    #[serde(rename = "exitDetail")]
    pub exit_detail: String,
    pub code: String,
    pub stdout: String,
    pub stderr: String,
}
