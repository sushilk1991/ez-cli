use serde::Serialize;
use std::fmt;

/// Structured JSON output envelope for all commands.
///
/// ## Versioning policy
/// - `version` increments only on breaking changes to JSON structure
/// - New fields can be added without a version bump
/// - Consumers should ignore unknown fields
#[derive(Serialize)]
pub struct CommandOutput {
    pub command: &'static str,
    pub version: u8,
    pub success: bool,
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl CommandOutput {
    pub fn new(command: &'static str, data: serde_json::Value) -> Self {
        Self {
            command,
            version: 1,
            success: true,
            data,
            metadata: None,
        }
    }

    #[allow(dead_code)]
    pub fn success(command: &'static str) -> Self {
        Self {
            command,
            version: 1,
            success: true,
            data: serde_json::json!({}),
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Typed error enum with granular exit codes.
pub enum EzError {
    /// Generic failure — exit code 1
    General(String),
    /// Bad arguments — exit code 2
    InvalidArgs(String),
    /// File/resource not found — exit code 3
    NotFound(String),
    /// Permission denied — exit code 4
    PermissionDenied(String),
    /// User cancelled or non-interactive abort — exit code 5
    Cancelled(String),
}

impl EzError {
    pub fn exit_code(&self) -> i32 {
        match self {
            EzError::General(_) => 1,
            EzError::InvalidArgs(_) => 2,
            EzError::NotFound(_) => 3,
            EzError::PermissionDenied(_) => 4,
            EzError::Cancelled(_) => 5,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "error": true,
            "code": self.exit_code(),
            "kind": match self {
                EzError::General(_) => "general",
                EzError::InvalidArgs(_) => "invalid_args",
                EzError::NotFound(_) => "not_found",
                EzError::PermissionDenied(_) => "permission_denied",
                EzError::Cancelled(_) => "cancelled",
            },
            "message": self.to_string(),
        })
    }
}

impl fmt::Display for EzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EzError::General(msg) => write!(f, "{}", msg),
            EzError::InvalidArgs(msg) => write!(f, "{}", msg),
            EzError::NotFound(msg) => write!(f, "{}", msg),
            EzError::PermissionDenied(msg) => write!(f, "{}", msg),
            EzError::Cancelled(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<String> for EzError {
    fn from(s: String) -> Self {
        EzError::General(s)
    }
}

/// Handle command result: print JSON or human error, set exit code.
pub fn output_result(json_mode: bool, _command: &'static str, result: Result<CommandOutput, EzError>) {
    match result {
        Ok(output) => {
            if json_mode {
                println!("{}", serde_json::to_string(&output).unwrap());
            }
            // Human output is already printed by the command itself
        }
        Err(e) => {
            if json_mode {
                eprintln!("{}", serde_json::to_string(&e.to_json()).unwrap());
            } else {
                eprintln!("{} {}", colored::Colorize::bold(&*colored::Colorize::red("Error:")), e);
            }
            std::process::exit(e.exit_code());
        }
    }
}
