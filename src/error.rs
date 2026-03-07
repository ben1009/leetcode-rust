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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn retry_exhausted(retries: u32, message: impl Into<String>) -> Self {
        Self::NetworkRetryExhausted {
            retries,
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_not_found_error() {
        let err = LeetCodeError::ProblemNotFound(42);
        let msg = err.to_string();
        assert!(msg.contains("Problem #42 not found"));
        assert!(msg.contains("paid-only"));
    }

    #[test]
    fn test_no_rust_version_error() {
        let err = LeetCodeError::NoRustVersion(100);
        assert!(err.to_string().contains("Problem #100 has no Rust version"));
    }

    #[test]
    fn test_already_initialized_error() {
        let err = LeetCodeError::AlreadyInitialized(1);
        assert!(
            err.to_string()
                .contains("Problem #1 has already been initialized")
        );
    }

    #[test]
    fn test_solution_exists_error() {
        let err = LeetCodeError::SolutionExists(5);
        assert!(
            err.to_string()
                .contains("Solution for problem #5 already exists")
        );
    }

    #[test]
    fn test_problem_not_exist_error() {
        let err = LeetCodeError::ProblemNotExist(10);
        assert!(
            err.to_string()
                .contains("Problem file for #10 doesn't exist")
        );
    }

    #[test]
    fn test_http_error() {
        let err = LeetCodeError::Http("connection refused".to_string());
        assert!(
            err.to_string()
                .contains("HTTP request failed: connection refused")
        );
    }

    #[test]
    fn test_http_client_build_error() {
        let err = LeetCodeError::HttpClientBuild("tls error".to_string());
        assert!(
            err.to_string()
                .contains("Failed to build HTTP client: tls error")
        );
    }

    #[test]
    fn test_invalid_input_error() {
        let err = LeetCodeError::InvalidInput("bad input".to_string());
        assert!(err.to_string().contains("Invalid input: bad input"));
    }

    #[test]
    fn test_template_error() {
        let err = LeetCodeError::Template("missing file".to_string());
        assert!(
            err.to_string()
                .contains("Template file error: missing file")
        );
    }

    #[test]
    fn test_async_runtime_error() {
        let err = LeetCodeError::AsyncRuntime("pool failed".to_string());
        assert!(err.to_string().contains("Async runtime error: pool failed"));
    }

    #[test]
    fn test_missing_title_slug_error() {
        let err = LeetCodeError::MissingTitleSlug(99);
        assert!(err.to_string().contains("Problem #99 has no title slug"));
    }

    #[test]
    fn test_metadata_parse_error() {
        let err = LeetCodeError::MetadataParse("invalid json".to_string());
        assert!(
            err.to_string()
                .contains("Failed to parse metadata: invalid json")
        );
    }

    #[test]
    fn test_code_definition_parse_error() {
        let err = LeetCodeError::CodeDefinitionParse("missing field".to_string());
        assert!(
            err.to_string()
                .contains("Failed to parse code definition: missing field")
        );
    }

    #[test]
    fn test_retry_exhausted_error() {
        let err = LeetCodeError::retry_exhausted(3, "timeout");
        let msg = err.to_string();
        assert!(msg.contains("Network error after 3 retries"));
        assert!(msg.contains("timeout"));
    }

    #[test]
    fn test_file_operation_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = LeetCodeError::file_operation("read", "/tmp/test.rs", io_err);
        let msg = err.to_string();
        assert!(msg.contains("Failed to read file at /tmp/test.rs"));
        assert!(msg.contains("file not found"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "no access");
        let err: LeetCodeError = io_err.into();
        assert!(err.to_string().contains("IO error: no access"));
    }

    #[test]
    fn test_from_regex_error() {
        // Create an invalid regex pattern to trigger an error
        // Using \x{0} which creates an invalid regex error at compile time in some cases,
        // but here we construct it dynamically to avoid compile-time errors
        let pattern = "a{2,1}";
        let regex_err = regex::Regex::new(pattern).unwrap_err();
        let err: LeetCodeError = regex_err.into();
        assert!(err.to_string().contains("Regex error"));
    }

    #[test]
    fn test_result_type_alias() {
        fn might_fail() -> Result<i32> {
            Ok(42)
        }
        assert_eq!(might_fail().unwrap(), 42);
    }
}
