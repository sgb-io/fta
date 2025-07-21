# Agent Guidelines for FTA Development

This document provides guidelines for AI agents (like GitHub Copilot) working on the FTA (Fast TypeScript Analyzer) codebase.

## Rust Code Formatting - CRITICAL

**ALWAYS run `cargo fmt --all` after making any Rust code changes.**

### Why This Matters
- The CI pipeline includes a formatting check: `cargo fmt --all -- --check`
- Unformatted Rust code will cause build failures
- Consistent formatting is enforced across the entire codebase

### Commands to Run

1. **After making Rust code changes:**
   ```bash
   cargo fmt --all
   ```

2. **To check formatting before committing:**
   ```bash
   cargo fmt --all -- --check
   ```

3. **To run clippy linting (recommended):**
   ```bash
   cargo clippy --all-features
   ```

## Development Workflow

When making changes to Rust code in this repository:

1. **Make your code changes**
2. **Format the code:** `cargo fmt --all`
3. **Check compilation:** `cargo check`
4. **Run tests:** `cargo test --all-features`
5. **Run clippy (optional but recommended):** `cargo clippy --all-features`

## Project Structure

This is a Rust workspace with the following crates:
- `crates/fta` - Core FTA analyzer library
- `crates/fta-wasm` - WebAssembly bindings

## CI/CD Requirements

The GitHub Actions workflows will:
- Check that code compiles (`cargo check`)
- Verify formatting (`cargo fmt --all -- --check`)
- Run all tests (`cargo test --all-features`)
- Build binaries for multiple platforms
- Maintain test coverage above 75%

## Key Reminders

- **Never commit unformatted Rust code** - it will break the build
- Use `cargo fmt --all` not just `cargo fmt` to format all workspace crates
- The `--check` flag only verifies formatting without making changes
- All Rust toolchain components (rustfmt, clippy) are already configured in CI

## Additional Notes

- This project analyzes TypeScript/JavaScript code but is written in Rust
- Code formatting consistency is critical for maintainability
- The CI pipeline is comprehensive and will catch formatting issues early