use super::{Channel, CrateType, Edition, Mode};
use serde::{Deserialize, Serialize};

/// Request structure for compiling Rust code via the playground API.
///
/// Contains configuration for target output, compilation channel, mode,
/// crate type, edition, and the source code to compile.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompileRequest {
    /// The output target format of the compilation (e.g., Assembly, MIR).
    pub target: CompileTarget,

    /// What flavour the assembly output should have (only required when target is [`CompileTarget::Assembly`])
    #[serde(rename = "assemblyFlavor")]
    pub assembly_flavor: Option<AssemblyFlavor>,

    /// If the assembly output should be demangled or not (only required when target is [`CompileTarget::Assembly`])
    #[serde(rename = "demangleAssembly")]
    pub demangle_assembly: Option<DemangleAssembly>,

    /// If the output should be processed or not (only required when target is [`CompileTarget::Assembly`])
    #[serde(rename = "processAssembly")]
    pub process_assembly: Option<ProcessAssembly>,

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

impl CompileRequest {
    /// Creates a new `CompileRequest` instance.
    ///
    /// # Arguments
    ///
    /// * `target` - The compilation target (e.g., Assembly, LLVM IR, Wasm).
    /// * `assembly_flavor` - The assembly flavor used when targeting assembly (e.g., AT&T or Intel). Optional.
    /// * `demangle_assembly` - Whether to demangle symbols in assembly output. Optional.
    /// * `process_assembly` - Whether to filter or output raw assembly. Optional.
    /// * `channel` - The Rust release channel (e.g., stable, beta, nightly).
    /// * `mode` - Compilation mode (e.g., debug or release).
    /// * `edition` - The Rust edition to compile against.
    /// * `crate_type` - The crate type (binary or library).
    /// * `tests` - Whether to compile with test harness.
    /// * `backtrace` - Whether to enable backtrace support.
    /// * `code` - The Rust source code to compile.
    ///
    /// # Returns
    ///
    /// A fully constructed `CompileRequest`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        target: CompileTarget,
        assembly_flavor: Option<AssemblyFlavor>,
        demangle_assembly: Option<DemangleAssembly>,
        process_assembly: Option<ProcessAssembly>,
        channel: Channel,
        mode: Mode,
        edition: Edition,
        crate_type: CrateType,
        tests: bool,
        backtrace: bool,
        code: String,
    ) -> Self {
        Self {
            target,
            assembly_flavor,
            demangle_assembly,
            process_assembly,
            channel,
            mode,
            edition,
            crate_type,
            tests,
            backtrace,
            code,
        }
    }
}

impl Default for CompileRequest {
    /// Provides a default `CompileRequest` configuration.
    ///
    /// Defaults to:
    /// - Target: `Assembly`
    /// - Assembly flavor: `AT&T`
    /// - Demangling: `Demangle`
    /// - Process: `Filter`
    /// - Channel: `Stable`
    /// - Mode: `Debug`
    /// - Edition: `2024`
    /// - Crate type: `Binary`
    /// - Tests: disabled
    /// - Backtrace: disabled
    /// - Code: `fn main() { println!("Hello, world!"); }`
    ///
    /// # Returns
    ///
    /// A `CompileRequest` instance with default values for compiling basic Rust code.
    fn default() -> Self {
        Self {
            target: CompileTarget::Assembly,
            assembly_flavor: Some(AssemblyFlavor::Att),
            demangle_assembly: Some(DemangleAssembly::Demangle),
            process_assembly: Some(ProcessAssembly::Filter),
            channel: Channel::Stable,
            mode: Mode::Debug,
            edition: Edition::Edition2024,
            crate_type: CrateType::Binary,
            tests: false,
            backtrace: false,
            code: "fn main() { println!(\"Hello, world!\"); }".to_owned(),
        }
    }
}

/// Response structure returned after compiling Rust code.
///
/// Includes compilation success status, process exit details, and outputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AssemblyFlavor {
    Att,
    Intel,
}

/// Determines whether assembly output is demangled or mangled.
///
/// - `Demangle`: Convert symbol names to human-readable form.
/// - `Mangle`: Keep symbol names mangled (default compiler format).
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DemangleAssembly {
    Demangle,
    Mangle,
}

/// Controls processing of assembly output.
///
/// - `Filter`: Filter assembly output for readability.
/// - `Raw`: Return raw assembly output without filtering.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessAssembly {
    Filter,
    Raw,
}

/// Defines the compilation target output format.
///
/// Variants:
/// - `Assembly`: Direct assembly output.
/// - `Hir`: High-level Intermediate Representation.
/// - `LlvmIr`: LLVM Intermediate Representation.
/// - `Mir`: Mid-level Intermediate Representation.
/// - `Wasm`: WebAssembly output.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CompileTarget {
    #[serde(rename = "asm")]
    Assembly,
    Hir,
    LlvmIr,
    Mir,
    Wasm,
}
