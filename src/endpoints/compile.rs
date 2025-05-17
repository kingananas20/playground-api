use super::{Channel, CrateType, Edition, Mode};
use serde::{Deserialize, Serialize};

/// Request structure for compiling Rust code via the playground API.
///
/// Contains configuration for target output, compilation channel, mode,
/// crate type, edition, and the source code to compile.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CompileRequest {
    /// The output target format of the compilation (e.g., Assembly, MIR).
    pub target: CompileTarget,

    /// The Rust release channel to use (stable, beta, nightly).
    pub channel: Channel,

    /// The compilation mode: debug or release.
    pub mode: Mode,

    /// The Rust edition to use (2015, 2018, 2021, 2024).
    pub edition: Edition,

    /// The crate type: binary or library.
    #[serde(rename = "crateType")]
    pub crate_type: CrateType,

    /// Whether to include test code during compilation.
    pub tests: bool,

    /// Whether to enable backtrace output on errors.
    pub backtrace: bool,

    /// The Rust source code to compile.
    pub code: String,
}

/// Response structure returned after compiling Rust code.
///
/// Includes compilation success status, process exit details, and outputs.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CompileResponse {
    /// Indicates if the compilation was successful.
    pub success: bool,

    /// Details about the compiler process exit (code, signals, etc.).
    #[serde(rename = "exitDetail")]
    pub exit_detail: String,

    /// The original source code sent for compilation.
    pub code: String,

    /// Standard output from the compiler.
    pub stdout: String,

    /// Standard error output, including warnings and errors.
    pub stderr: String,
}

/// Specifies the assembly syntax flavor for assembly output.
///
/// - `Att`: AT&T syntax (common on Unix-like systems).
/// - `Intel`: Intel syntax (common on Windows).
#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AssemblyFlavor {
    Att,
    Intel,
}

/// Determines whether assembly output is demangled or mangled.
///
/// - `Demangle`: Convert symbol names to human-readable form.
/// - `Mangle`: Keep symbol names mangled (default compiler format).
#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DemangleAssembly {
    Demangle,
    Mangle,
}

/// Controls processing of assembly output.
///
/// - `Filter`: Filter assembly output for readability.
/// - `Raw`: Return raw assembly output without filtering.
#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessAssembly {
    Filter,
    Raw,
}

/// Defines the compilation target output format.
///
/// Variants:
/// - `Assembly` with options for syntax, demangling, and processing.
/// - `Hir`: High-level Intermediate Representation.
/// - `LlvmIr`: LLVM Intermediate Representation.
/// - `Mir`: Mid-level Intermediate Representation.
/// - `Wasm`: WebAssembly output.
#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CompileTarget {
    Assembly(AssemblyFlavor, DemangleAssembly, ProcessAssembly),
    Hir,
    LlvmIr,
    Mir,
    Wasm,
}
