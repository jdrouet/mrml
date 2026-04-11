# How to Contribute

Don't hesitate to take any issue that has been marked as valid. Considering that I'm not a native English speaker, any language fixing pull request is welcome!

If you have a new feature in mind, please [contact me](mailto:jeremie.drouet@gmail.com), I'd be happy to discuss your needs and expectations.

## Requirements

1. Pick a [good issue to start with](https://github.com/jdrouet/mrml/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
2. Create a branch and make your changes. Remember to sign off your commits using `git commit -sm "module: your commit message"`
3. Make sure you improve the product and its codebase
4. Create a [pull request](https://opensource.guide/how-to-contribute/#opening-a-pull-request)
5. Fix things that come up during review
6. Wait for it to get merged!

## Commit Message Convention

We use [Conventional Commits](https://www.conventionalcommits.org/) for commit messages. Prefix your commit message with the module name.

**Allowed types:** `ci`, `chore`, `build`, `doc`, `example`, `feat`, `fix`, `perf`, `refactor`, `revert`, `style`, `test`

**Allowed scopes:** `deps`, `deps-dev`, `mrml-cli`, `mrml-core`, `mrml-python`, `mrml-wasm`

**Examples:**
- `feat(mrml-core): add support for new mj-component`
- `fix(mrml-cli): handle empty input files`
- `docs: update contributing guide`
- `build(deps): bump serde to 1.0.200`

---

# Project Architecture

## Overview

**MRML** is a Rust reimplementation of [MJML](https://mjml.io/) (Mailjet Markup Language), an email templating framework. The project is organized as a Cargo workspace monorepo with multiple crates targeting different platforms.

## Repository Structure

```
mrml/
├── packages/
│   ├── mrml-core/           # Core library - MJML parsing and rendering engine
│   │   ├── src/             # Source code with MJML component modules
│   │   ├── tests/           # Integration tests
│   │   ├── benches/         # Criterion benchmarks
│   │   └── lib/             # Helper libraries (html-compare, css-compare)
│   ├── mrml-cli/            # Command-line interface tool
│   ├── mrml-python/         # Python bindings via PyO3
│   └── mrml-wasm/           # WebAssembly bindings for browser/Node.js
├── examples/
│   ├── axum/                # Axum web framework example
│   └── demo/                # Interactive demo application
├── .github/workflows/       # CI/CD workflows
├── Cargo.toml               # Workspace configuration
├── rustfmt.toml             # Code formatting configuration
└── cliff.toml               # Changelog generator configuration
```

## Crate Dependencies

```
mrml-core (library)
    ├── mrml-cli (depends on mrml-core)
    ├── mrml-python (depends on mrml-core)
    └── mrml-wasm (depends on mrml-core)
```

## mrml-core Module Structure

Each MJML component follows a consistent module pattern in `packages/mrml-core/src/`:

```
mj_<component>/
├── mod.rs      # Component type definition and module exports
├── parse.rs    # MJML parsing logic (feature: parse)
├── render.rs   # HTML rendering logic (feature: render)
├── print.rs    # MJML printing/serialization (feature: print)
└── json.rs     # JSON serialization (feature: json)
```

**Key modules:**
- `mj_body`, `mj_head`, `mjml` - Root document structure
- `mj_section`, `mj_column`, `mj_group` - Layout components
- `mj_text`, `mj_button`, `mj_image`, `mj_table` - Content components
- `mj_accordion`, `mj_carousel`, `mj_navbar`, `mj_social` - Interactive components
- `mj_include` - Template inclusion support
- `prelude/` - Shared traits and utilities (`Component`, `StaticTag`, `AttributeMap`)

## Feature Flags (mrml-core)

The core library uses Cargo features for conditional compilation:

| Feature | Description |
|---------|-------------|
| `json` | JSON serialization/deserialization support |
| `parse` | MJML template parsing |
| `print` | MJML template printing |
| `render` | HTML rendering |
| `async` | Async/await support |
| `local-loader` | File-based template loading for mj-include |
| `http-loader-ureq` | HTTP resource loading (blocking, via ureq) |
| `http-loader-async-reqwest` | HTTP resource loading (async, via reqwest) |
| `css-inline` | CSS inlining support |

**Default features:** `json`, `parse`, `print`, `render`

---

# Development Setup

## Prerequisites

- **Rust:** Install via [rustup](https://rustup.rs/) (stable toolchain required, nightly for formatting)
- **Python 3.7+:** Required for mrml-python development
- **Node.js:** Required for mrml-wasm development
- **wasm-pack:** Required for mrml-wasm (`curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`)

## Building

```bash
# Build entire workspace
cargo build

# Build specific package
cargo build -p mrml           # Core library
cargo build -p mrml-cli       # CLI tool
cargo build -p mrml-python    # Python bindings (use maturin instead)
cargo build -p mrml-wasm      # WASM (use wasm-pack instead)

# Build with specific features
cargo build -p mrml --features "json,parse,render"

# Build with all features
cargo build -p mrml --all-features

# Release build
cargo build --release -p mrml-cli
```

## Testing

```bash
# Run all tests in workspace (excludes Python and WASM)
cargo test --workspace --exclude mrml-python --exclude mrml-wasm

# Run tests for specific package
cargo test -p mrml              # Core library tests
cargo test -p mrml-cli          # CLI tests

# Run tests with all features enabled
cargo test -p mrml --all-features

# Run a specific test
cargo test -p mrml test_name

# Run tests with output
cargo test -p mrml -- --nocapture
```

### Testing mrml-python

```bash
cd packages/mrml-python
python3 -m venv env
source env/bin/activate
pip install pytest maturin
maturin develop
python3 -m pytest
```

### Testing mrml-wasm

```bash
cd packages/mrml-wasm
wasm-pack test --node
```

## Code Quality

### Formatting

```bash
# Check formatting (requires nightly)
cargo +nightly fmt --all --check

# Apply formatting
cargo +nightly fmt --all
```

**rustfmt.toml configuration:**
- `imports_granularity = "Module"` - One import per module
- `group_imports = "StdExternalCrate"` - Group std, external, then crate imports
- `reorder_imports = true` - Sort imports alphabetically

### Linting

```bash
# Run clippy on entire workspace
cargo clippy --all-targets --all-features --tests --workspace

# Run clippy with warnings as errors (CI mode)
RUSTFLAGS="-Dwarnings" cargo clippy --all-targets --all-features --tests --workspace
```

### Security Audit

```bash
cargo audit
```

## Benchmarks

```bash
cd packages/mrml-core
cargo bench
```

Benchmarks use [Criterion](https://github.com/bheisler/criterion.rs) and are located in `packages/mrml-core/benches/`.

---

# CI/CD Workflows

All workflows are in `.github/workflows/`. Key workflows:

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `code-checking.yml` | PR, push to main | Format check, clippy, cargo check |
| `mrml-core-main.yml` | Changes to mrml-core | Run tests with coverage |
| `mrml-cli-main.yml` | Changes to mrml-cli | Run CLI tests, track binary size |
| `mrml-python-main.yml` | Changes to mrml-python | Multi-platform Python tests |
| `mrml-wasm-main.yml` | Changes to mrml-wasm | WASM build and integration tests |
| `commit-lint.yml` | PR | Validate conventional commit format |
| `security-audit.yml` | Cargo.toml/lock changes | Security vulnerability scan |

---

# Making Changes

## Adding a New MJML Component

1. Create a new module in `packages/mrml-core/src/mj_<component>/`
2. Implement the standard files: `mod.rs`, `parse.rs`, `render.rs`, `print.rs`, `json.rs`
3. Export the module in `packages/mrml-core/src/lib.rs`
4. Add the component to parent component children enums (e.g., `MjBodyChild`)
5. Add tests in the module or in `packages/mrml-core/tests/`

## Fixing a Rendering Bug

When mrml produces different HTML output than the official mjml library, the fix involves adding a comparison test, generating the expected output, and updating the Rust rendering code.

### 1. Create the MJML test template

Add a `.mjml` file in `packages/mrml-core/resources/compare/success/` that reproduces the issue:

```bash
# Example: packages/mrml-core/resources/compare/success/mj-text-custom-case.mjml
cat > packages/mrml-core/resources/compare/success/mj-text-custom-case.mjml << 'EOF'
<mjml>
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-text>Your test case here</mj-text>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>
EOF
```

### 2. Generate the expected HTML with mjml

Run the transpilation script to generate the expected `.html` file from the official mjml npm package:

```bash
cd scripts
npm install    # first time only
cd ..
node scripts/update-resources.js
```

This compiles every `.mjml` file in the comparison directories using the official mjml library, then applies cleanup transformations (ID normalization, whitespace normalization, rounding adjustments). The generated `.html` file is the expected output.

**Manual adjustments.** Sometimes the generated HTML needs tweaking because the `pretty` formatter introduces artifacts. Known cases are already handled in the script's `cleanup()` function, but new cases may need a manual fix or a new cleanup rule in the script.

### 3. Register the test

Add a `should_render!` macro invocation in the component's `render.rs` test module:

```rust
// In packages/mrml-core/src/mj_text/render.rs
#[cfg(test)]
mod tests {
    crate::should_render!(custom_case, "mj-text-custom-case");
}
```

The macro generates both sync and async tests that:
1. Parse the `.mjml` template
2. Render it to HTML
3. Compare the result against the `.html` file using `html_compare::assert_similar`

The comparison is smart -- it ignores whitespace differences, CSS property ordering, and class ordering.

### 4. Run the failing test

```bash
cargo test -p mrml --all-features "custom_case_sync"
```

### 5. Fix the rendering code and verify

Fix the Rust rendering code (typically in `src/mj_<component>/render.rs`), then run the full test suite:

```bash
cargo test -p mrml --all-features
```

### Regression tests for parsing bugs

For bugs that aren't about rendering differences, create a standalone test file:

1. Create `packages/mrml-core/tests/issue-<number>.rs`
2. Write a failing test that reproduces the issue
3. Fix the bug
4. Verify the test passes

## Modifying the CLI

The CLI is in `packages/mrml-cli/`. It uses:
- `clap` for argument parsing
- `mrml` with `http-loader-ureq` and `local-loader` features

## Working with Python Bindings

Located in `packages/mrml-python/`:
- Uses PyO3 for Rust-Python interop
- Build with `maturin develop` for local testing
- Tests are in `packages/mrml-python/tests/`

## Working with WASM Bindings

Located in `packages/mrml-wasm/`:
- Uses `wasm-bindgen` for JS interop
- Build with `wasm-pack build --target <web|nodejs|bundler>`
- Run `bash build.sh` for full build
- Examples in `packages/mrml-wasm/examples/`

---

# Quick Reference Commands

```bash
# Full CI check (what CI runs)
cargo +nightly fmt --all --check
RUSTFLAGS="-Dwarnings" cargo check --all-features --tests --workspace
RUSTFLAGS="-Dwarnings" cargo clippy --all-targets --all-features --tests --workspace
cargo test --workspace --exclude mrml-python --exclude mrml-wasm --all-features

# Quick development cycle
cargo fmt --all                    # Format code
cargo clippy --all-features        # Lint
cargo test -p mrml                 # Test core
cargo test -p mrml-cli             # Test CLI

# Build release artifacts
cargo build --release -p mrml-cli  # CLI binary

# Python development
cd packages/mrml-python && maturin develop && pytest

# WASM development  
cd packages/mrml-wasm && wasm-pack test --node
```

---

# Important Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Workspace configuration and shared dependencies |
| `rustfmt.toml` | Code formatting rules |
| `cliff.toml` | Changelog generation configuration |
| `release-plz.toml` | Release automation configuration |
| `.editorconfig` | Editor settings (2-space indent default, 4-space for Rust) |
| `packages/mrml-core/Cargo.toml` | Core library dependencies and features |
| `packages/mrml-core/src/lib.rs` | Public API exports |
| `packages/mrml-core/src/prelude/mod.rs` | Shared traits and utilities |
