# Contributing to Toll Optimizer

Thanks for considering a contribution! This is a small, focused project, so the
bar is simple: keep it tested, keep it documented, keep it offline.

## Ways to contribute

- **Code**: bug fixes and small features via pull requests.
- **Rate tables**: 407 ETR publishes new rates annually. Updating the tables in
  `src/vehicle_class/` and `src/constants.rs` — and bumping
  `NEWEST_RATE_TABLE_YEAR` in `src/trip_analyzer.rs` — is one of the most
  valuable contributions you can make.
- **Unrecognized access points / vehicle classes**: if the tool reports something
  as NOT RECOGNIZED, open an issue with the exact name from your statement.
- **Docs**: README, `docs/`, and `SKILL.md` improvements are welcome.

## Getting started

You need a recent stable Rust toolchain (`rustup` recommended).

```bash
cargo build   # debug build
cargo test    # full suite: unit + integration + e2e
cargo bench   # benchmarks
```

## Quality gates (CI enforces all of these)

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo doc --no-deps
```

Run all four before opening a PR. CI runs on every push to `main` and every
pull request.

## Code standards

- Rustdoc on all public items, with usage examples where helpful.
- `Result` for fallible operations; no `unwrap()`/`expect()` in library code.
- No new dependencies without explaining why in the PR description.
- Keep the binary offline: no network calls, no telemetry, no data collection.

## Tests

- New behavior needs tests — unit tests in `src/` or integration tests in `tests/`.
- Test CSVs must use synthetic data only. Never commit a real statement;
  see `tests/csv/` for the anonymized fixtures.

## Pull requests

1. Fork, branch from `main`, keep the change focused.
2. Update `README.md` / `docs/` if behavior or flags changed.
3. Make sure the four quality gates pass.
4. Describe what changed and why in the PR body.

## Releases (maintainers)

Releases are cut by pushing a `v*` tag (e.g. `v1.0.6`). GitHub Actions builds
the Linux, macOS (Intel + Apple Silicon), and Windows binaries and attaches
`SHA256SUMS.txt` automatically.

## Reporting issues

Include the command you ran, the full output, your OS, and
`toll-optimizer --version`. For CSV parsing problems, attach a redacted snippet
— remove names, plate numbers, and account numbers first.
