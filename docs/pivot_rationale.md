# Plan: Pivot to Standalone CLI & Distribution

This document outlines the strategy to transform the **Toll Optimizer** from a full-stack web application into a professional, standalone CLI tool distributed for multiple platforms.

## Background & Motivation
The project currently includes a web UI, REST API, database, and monitoring stack. To simplify distribution and focus on the core value proposition (analyzing toll statements), we are pivoting to a CLI-only model. This ensures users can run the tool without installing Rust, Cargo, or any database infrastructure.

## Distribution Strategy
We will follow the industry standard for Rust CLI tools by using a CI/CD pipeline (GitHub Actions) to automatically build binaries for multiple platforms whenever a "Release" is created.

### Target Platforms:
*   **Linux**: Build a statically linked binary using the `x86_64-unknown-linux-musl` target (ensures it runs on any Linux distro without dependency issues).
*   **Windows**: Build an `.exe` using the `x86_64-pc-windows-msvc` target.
*   **macOS**: Build "Universal" binaries (supporting both Intel and Apple Silicon/M1/M2) using `x86_64-apple-darwin` and `aarch64-apple-darwin`.

---

## Phase 1: Archiving & Cleanup
1.  **Archive Full-Stack Code**:
    *   Ensure `main` is clean.
    *   Create an archive branch: `git checkout -b archive/web-app-v1`.
    *   Push to remote: `git push -u origin archive/web-app-v1`.
2.  **Prune `main` Branch**:
    *   Return to main: `git checkout main`.
    *   Delete: `frontend/`, `monitoring/`, `.sqlx/`, `crates/api/`, `docker-compose.yml`, `Dockerfile`, `Caddyfile`, `env.txt`.
3.  **Update `.gitignore`**:
    *   Remove entries related to Node.js, Docker, and PostgreSQL.

## Phase 2: Implementation & Restructuring
1.  **Refactor for Portability**: 
    *   Ensure the CLI code is completely decoupled from the API and database.
    *   Move `crates/core/src/*` to `src/` and `crates/cli/src/main.rs` to `src/main.rs`.
    *   The root crate will only depend on the logic previously in `crates/core`.
2.  **Add a CLI Framework**:
    *   Integrate `clap` (with the `derive` feature) into the root `Cargo.toml`.
    *   This provides professional-grade argument parsing (e.g., `--help`, versioning, and pretty error messages).
3.  **Flatten Project Structure**:
    *   Merge dependencies into a single root `Cargo.toml`.
    *   Remove the `[workspace]` definition.
    *   Delete the `crates/` directory.

## Phase 3: Release Workflow (CI/CD)
Create a `.github/workflows/release.yml` file that:
1.  **Triggers on a new Git tag** matching `v*` (e.g., `v1.0.0`).
2.  **Compiles the binaries** for Windows, macOS (Intel & Silicon), and Linux using the targets specified above.
3.  **Compresses them** into `.zip` or `.tar.gz` files.
4.  **Uploads them** to a GitHub Release page automatically.

### Example Workflow Snippet:
```yaml
name: Release
on:
  push:
    tags: ['v*']
jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
    steps:
      - uses: actions/checkout@v4
      - run: cargo build --release --target ${{ matrix.target }}
      # Packaging and Upload steps go here
```

## Phase 4: Documentation
1.  **README.md**:
    *   Focus entirely on CLI installation and usage.
    *   Add clear "Download" links pointing to the GitHub Releases page.
    *   Document how to build from source using `cargo install --path .`.

## Verification Checklist
- [ ] `cargo check` passes on the new root structure.
- [ ] `cargo test` confirms core logic integrity.
- [ ] Verify binary is statically linked on Linux (`ldd` check).
- [ ] Test help output: `toll-optimizer --help`.
