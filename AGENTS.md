# LeetCode Rust - Project Guide for AI Agents

## Project Overview

This is a LeetCode solution development toolkit written in Rust. It provides:

1. **Interactive CLI tool** (`cargo run`) to fetch LeetCode problems and generate Rust solution templates
2. **Problem organization** with separate directories for in-progress (`problem/`) and completed (`solution/`) solutions
3. **Utility modules** for common LeetCode data structures (ListNode, TreeNode, Point)
4. **Automated testing** infrastructure using cargo-nextest and coverage reporting

## Technology Stack

- **Language**: Rust (Edition 2024)
- **Toolchain**: Nightly (`nightly-2025-12-01`)
- **Task Runner**: cargo-make
- **Test Runner**: cargo-nextest
- **Coverage**: cargo-llvm-cov
- **HTTP Client**: reqwest (blocking), surf (async)
- **Serialization**: serde, serde_json

## Project Structure

```
├── Cargo.toml              # Rust package manifest
├── rust-toolchain          # Nightly toolchain specification
├── rustfmt.toml           # Code formatting configuration
├── Makefile.toml          # cargo-make task definitions
├── template.rs            # Problem template file used by generator
├── .env                   # Environment variables (LEETCODE_COOKIE)
├── .typos.toml           # Typo checker configuration
├── dev                   # Setup script for cargo-make
└── src/
    ├── main.rs           # CLI entry point - problem fetcher/generator
    ├── lib.rs            # Library root
    ├── fetcher.rs        # LeetCode API client (GraphQL + REST)
    ├── problem/          # In-progress problem solutions
    │   ├── mod.rs        # Module declarations for all problems
    │   └── p{xxxx}_{title}.rs  # Individual problem files
    ├── solution/         # Completed solutions (optional)
    │   └── mod.rs        # Module declarations (referenced but empty)
    └── util/             # Helper utilities
        ├── mod.rs
        ├── linked_list.rs    # ListNode and macros (linked!, list!)
        ├── tree.rs           # TreeNode and macros (tree!)
        ├── point.rs          # Point struct
        └── vec_string.rs     # Vec<String> helpers
```

## Build and Development Commands

### Prerequisites

1. Set `LEETCODE_COOKIE` in `.env` file or environment variable (LeetCode session cookie for API access)
2. Run `./dev` to install cargo-make (or install manually: `cargo install cargo-make`)

### CLI Usage

The CLI now uses subcommands with proper argument parsing (via `clap`):

```bash
# Generate template for a specific problem
cargo run -- get <problem_id>
leetcode-rust get <problem_id>    # if installed

# Generate a random problem template
cargo run -- random
leetcode-rust random

# Initialize ALL problems (warning: fetches all LeetCode problems!)
cargo run -- all
leetcode-rust all

# Move a problem from problem/ to solution/ when completed
cargo run -- solve <problem_id>
leetcode-rust solve <problem_id>

# List all initialized problems
cargo run -- list
leetcode-rust list

# Get help
cargo run -- --help
cargo run -- get --help
```

### Legacy Interactive Mode (OLD - No longer supported)

The old interactive mode has been replaced with proper CLI arguments for better UX and scripting support.

### Test Commands

```bash
# Test a specific solution
cargo test test_{problem_id}

# Run all tests
cargo nextest run --workspace --all-features --all-targets
```

### cargo-make Tasks (via `./dev` or `cargo make`)

| Task | Description |
|------|-------------|
| `check` | Run all checks (fmt, deps, clippy, machete, test, typos) |
| `test` | Run unit tests with cargo-nextest |
| `test-cov` | Run tests and generate HTML coverage report |
| `check-fmt` | Check code formatting |
| `check-clippy` | Run clippy with warnings as errors |
| `check-typos` | Run typo checker |
| `check-machete` | Check for unused dependencies |
| `check-dep-sort` | Check dependency sorting in Cargo.toml |
| `clean` | Run cargo clean |

## Code Style Guidelines

### Formatting (rustfmt.toml)

- **Edition**: 2021
- **Style Edition**: 2024
- **Comment Width**: 120 characters
- **Tab Spaces**: 4
- **Import Organization**: Group by `StdExternalCrate`
- **Imports Granularity**: Crate-level
- **Format doc comments**: Enabled
- **Normalize comments**: Enabled
- **Wrap comments**: Enabled

### Problem File Structure

Each problem file follows this template:

```rust
/**
 * [{problem_id}] {Problem Title}
 *
 * {Problem description}
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/{title-slug}/
// discuss: https://leetcode.com/problems/{title-slug}/discuss/...

// submission codes start here

impl Solution {
    pub fn solution_method(...) -> ... {
        // Implementation
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_{problem_id}() {
        // Test cases
    }
}
```

### Naming Conventions

- Problem files: `p{xxxx}_{snake_case_title}.rs` (e.g., `p0001_two_sum.rs`)
- Solution files: `s{xxxx}_{snake_case_title}.rs` (e.g., `s0001_two_sum.rs`)
- Test functions: `test_{problem_id}` (e.g., `test_1`)
- Module declarations in `problem/mod.rs`: `pub mod p{xxxx}_{title};`

## Testing Instructions

### Running Tests

```bash
# All tests
cargo nextest run

# Specific problem test
cargo test test_1  # for problem #1

# With coverage
cargo llvm-cov nextest --workspace --all-features --all-targets --html
```

### Test Structure

Tests are co-located in each problem file using `#[cfg(test)]` modules. The project uses:

- **Standard Rust testing** with `#[test]` attribute
- **cargo-nextest** as the test runner in CI (faster, better output)
- **Test utilities** from `util/` module for creating test data

### Utility Macros for Testing

```rust
// Linked lists
linked![1, 2, 3]  // Create linked list from values
list![1, 2, 3]    // Alternative syntax

// Trees
tree![1, 2, 3]    // Create binary tree from level-order values
```

## CI/CD and Automation

### GitHub Actions Workflows

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `check.yml` | PR, push to main | fmt, clippy, typos checks |
| `test.yml` | PR, push to main | Test on Ubuntu/macOS/Windows + coverage |
| `safety.yml` | Scheduled | Security auditing |
| `scheduled.yml` | Scheduled | Dependency updates |
| `scorecards.yml` | PR, push | OpenSSF Scorecard |
| `dependency-review.yml` | PR | Dependency security review |

### Security Hardening

- Uses `step-security/harden-runner` in CI workflows
- Codecov integration for coverage reporting
- OpenSSF Scorecard for security metrics

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `backon` | Exponential backoff retry for network requests |
| `clap` | CLI argument parsing with derive macros |
| `dotenv` | Environment variable loading |
| `futures` | Async runtime for bulk problem fetching |
| `indicatif` | Progress bars for long operations |
| `rand` | Random problem selection |
| `regex` | Text processing in template generation |
| `reqwest` | HTTP client for LeetCode API |
| `serde` / `serde_json` | JSON serialization |
| `thiserror` | Structured error handling |
| `tokio` | Async runtime for retry delays |

## Error Handling

The CLI uses structured error handling with `thiserror`:

- **User-friendly messages**: No more panics - all errors display helpful messages
- **Retry logic**: Network requests automatically retry with exponential backoff using `backon` crate (3 retries)
- **Specific error types**: `ProblemNotFound`, `NoRustVersion`, `AlreadyInitialized`, etc.

Example error output:
```
❌ Error: Problem #99999 not found (may be paid-only or doesn't exist)
```

### Retry Implementation

The fetcher uses `backon` for resilient network requests:

```rust
// Blocking retry
(|| fetch_problem_detail(...))
    .retry(ExponentialBuilder::default().with_max_times(3))
    .call()

// Async retry
(|| async { fetch_problem_detail_async(...).await })
    .retry(ExponentialBuilder::default().with_max_times(3))
    .sleep(tokio::time::sleep)
    .await
```

## Development Notes

### Adding a New Problem

1. Run `cargo run -- get {problem_id}` to fetch and generate template
2. Implement solution in the generated file
3. Add test cases in the `#[cfg(test)]` module
4. Run `cargo test test_{problem_id}` to verify
5. Optionally move to solution/: `cargo run -- solve {problem_id}`

### Working with LeetCode API

The fetcher (`src/fetcher.rs`) uses:
- **GraphQL endpoint** (`https://leetcode.com/graphql`) for problem details
- **REST API** (`https://leetcode.com/api/problems/algorithms/`) for problem list
- Requires `LEETCODE_COOKIE` environment variable (LeetCode session cookie)

### Custom Data Structures

The project provides Rust implementations of common LeetCode structures:

- **`ListNode`** - Singly linked list with `val: i32` and `next: Option<Box<ListNode>>`
- **`TreeNode`** - Binary tree with `Rc<RefCell<TreeNode>>` for shared ownership
- **`Point`** - 2D point structure

These are automatically imported in generated templates when needed based on return types.

## Important Files for Agents

- **Template**: `template.rs` - Used to generate new problem files
- **Module Registry**: `src/problem/mod.rs` - Add `pub mod` declarations here
- **Formatting**: `rustfmt.toml` - Run `cargo fmt --all` before committing
- **Tasks**: `Makefile.toml` - Use `./dev <task>` for development tasks
