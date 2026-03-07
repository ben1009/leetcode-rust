# 🦀 Leetcode Solution in Rust, just to torture myself for fun 🤪

[![codecov](https://codecov.io/gh/ben1009/leetcode-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/ben1009/leetcode-rust)
[![Check](https://github.com/ben1009/leetcode-rust/actions/workflows/check.yml/badge.svg?branch=master)](https://github.com/ben1009/leetcode-rust/actions/workflows/check.yml)

Set `LEETCODE_COOKIE` in `.env` file or environment variable first.

## CLI Usage

The CLI uses subcommands for better UX:

```bash
# Get a specific problem by ID
cargo run -- get <id>

# Get a random problem
cargo run -- random

# Initialize ALL problems (warning: fetches all problems!)
cargo run -- all

# Move a problem from problem/ to solution/
cargo run -- solve <id>

# List all initialized problems
cargo run -- list

# Get help
cargo run -- --help
```

## Testing

```bash
# Test a specific solution
cargo test test_<id>

# Run all tests
cargo nextest run
```

## Features

- ✅ **User-friendly CLI** with subcommands and help messages
- ✅ **Auto-retry** with exponential backoff for network requests
- ✅ **Progress indicators** for long-running operations
- ✅ **Structured error handling** - no more panics!

## Quick Start

1. Set `LEETCODE_COOKIE` in `.env` file
2. Run `cargo run -- get 1` to fetch problem #1
3. Implement your solution in the generated file
4. Run `cargo test test_1` to verify
# Mergify label test
