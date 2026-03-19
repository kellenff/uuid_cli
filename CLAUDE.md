# CLAUDE.md

This file provides guidance for AI assistants working in this codebase.

## Project Overview

**uuid_cli** is a lightweight Rust command-line utility for generating UUIDs (Universally Unique Identifiers). It produces one or more UUIDs with configurable formatting options.

Binary installed as `uuid`. Current version: `0.1.1`.

## Repository Structure

```
uuid_cli/
├── Cargo.toml          # Package manifest and dependencies
├── Cargo.lock          # Locked dependency versions
├── README.md           # User-facing documentation
├── code-of-conduct.txt # Contributor Covenant v1.4
└── src/
    ├── main.rs         # Binary entry point (CLI arg parsing + output)
    └── lib.rs          # Core logic (formatting, enums, clap app config)
```

## Build & Development Commands

```bash
# Build the project
cargo build

# Build optimized release binary
cargo build --release

# Run the binary directly
cargo run -- [args]

# Run all tests
cargo test

# Run tests with stdout output
cargo test -- --nocapture

# Check for lint issues
cargo clippy

# Format code
cargo fmt
```

## Dependencies

| Crate       | Version  | Purpose                          |
|-------------|----------|----------------------------------|
| uuid        | ^0.8     | UUID v4 generation               |
| clap        | ^2.33    | CLI argument parsing             |
| serde_json  | 1.0.41   | JSON output serialization        |

Rust edition: **2018**. Toolchain: rustc/cargo 1.93.1.

## Code Architecture

### `src/main.rs`
Entry point. Calls `get_app()` from lib, parses matches, generates UUIDs via the `uuid` crate, then delegates formatting to `format_uuids()`.

### `src/lib.rs`
All library logic:
- **`OutputFormat` enum** — `Unix` (newline-separated) or `Json` (JSON array). Implements `FromStr` for CLI parsing.
- **`UuidFormat` enum** — `Lower` or `Upper` case.
- **`get_app()`** — Constructs the `clap::App` with all CLI arguments.
- **`format_uuids()`** — Takes a `Vec<Uuid>`, `OutputFormat`, and `UuidFormat`; returns a formatted `String`.
- **`format_uuid()`** — Private helper converting a single `Uuid` to the desired case.

## CLI Interface

```
uuid [FLAGS] [OPTIONS]

Options:
  -c, --count <count>     Number of UUIDs to generate [default: 1]
  --format <format>       Output format: unix (default) or json
  --upper                 Output UUIDs in uppercase
  --lower                 Output UUIDs in lowercase (default)
```

Examples:
```bash
uuid                    # Generate one UUID
uuid -c 5               # Generate five UUIDs
uuid --format=json      # JSON array output
uuid --upper            # Uppercase output
```

## Testing

Tests live in `src/lib.rs` under `#[cfg(test)]`. Current coverage:
- `format_uuid_lowercase` — verifies `UuidFormat::Lower` produces lowercase output
- `format_uuid_uppercase` — verifies `UuidFormat::Upper` produces uppercase output

When adding new features, add corresponding unit tests in `lib.rs`. Follow the pattern of calling the function under test directly with controlled inputs.

## Code Conventions

- Follow standard Rust naming: `snake_case` for functions/variables, `PascalCase` for types/enums.
- Run `cargo clippy` before committing; the project enforces clippy compliance.
- Run `cargo fmt` to format code consistently.
- Prefer `.expect("descriptive message")` over bare `.unwrap()` where failure is possible and context helps.
- Keep `main.rs` thin — orchestration only. Business logic belongs in `lib.rs`.

## Git Workflow

- The default branch is `master`.
- Feature branches use the format `claude/<description>-<id>` for AI-assisted work.
- Commit messages should be short, imperative, and descriptive.
- Commits are signed with SSH keys (configured via `commit.gpgsign=true`).
- Push with: `git push -u origin <branch-name>`

## No CI/CD

There is no automated CI pipeline. Run `cargo test` and `cargo clippy` locally before pushing.
