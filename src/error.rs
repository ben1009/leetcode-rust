//! Error types for leetcode-rust CLI

use std::path::PathBuf;

use thiserror::Error;

/// Result type alias with LeetCodeError
pub type Result<T> = std::result::Result<T, LeetCodeError>;

/// Main error type for LeetCode CLI operations
#[derive(Error, Debug)]
pub enum LeetCodeError {
    /// IO operation failed
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(String),

    /// Failed to parse JSON response
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// Problem not found
    #[error("Problem #{0} not found (may be paid-only or doesn't exist)")]
    ProblemNotFound(u32),

    /// No Rust version available for problem
    #[error("Problem #{0} has no Rust version available")]
    NoRustVersion(u32),

    /// Problem already initialized
    #[error("Problem #{0} has already been initialized in problem/")]
    AlreadyInitialized(u32),

    /// Solution already exists
    #[error("Solution for problem #{0} already exists in solution/")]
    SolutionExists(u32),

    /// Problem file doesn't exist for solving
    #[error("Problem file for #{0} doesn't exist in problem/")]
    ProblemNotExist(u32),

    /// Environment variable not set
    #[allow(dead_code)]
    #[error("Environment variable {0} not set")]
    EnvVar(String),

    /// Failed to build HTTP client
    #[error("Failed to build HTTP client: {0}")]
    HttpClientBuild(String),

    /// Network error with retry exhaustion
    #[error("Network error after {retries} retries: {message}")]
    NetworkRetryExhausted { retries: u32, message: String },

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Template file error
    #[error("Template file error: {0}")]
    Template(String),

    /// Regex error
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    /// File operation error with path
    #[error("Failed to {operation} file at {path}: {source}")]
    FileOperation {
        operation: String,
        path: PathBuf,
        source: std::io::Error,
    },

    /// Async runtime error
    #[error("Async runtime error: {0}")]
    AsyncRuntime(String),

    /// Missing problem title slug
    #[error("Problem #{0} has no title slug")]
    MissingTitleSlug(u32),

    /// Missing metadata
    #[error("Failed to parse metadata: {0}")]
    MetadataParse(String),

    /// Missing code definition
    #[error("Failed to parse code definition: {0}")]
    CodeDefinitionParse(String),
}

impl LeetCodeError {
    /// Create a file operation error
    pub fn file_operation(
        operation: &str,
        path: impl Into<PathBuf>,
        source: std::io::Error,
    ) -> Self {
        Self::FileOperation {
            operation: operation.to_string(),
            path: path.into(),
            source,
        }
    }

    /// Create a network retry exhausted error
    pub fn retry_exhausted(retries: u32, message: impl Into<String>) -> Self {
        Self::NetworkRetryExhausted {
            retries,
            message: message.into(),
        }
    }
}
