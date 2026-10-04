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
cargo build                  # debug build
cargo test                   # full suite: unit + integration + e2e
cargo bench                  # benchmarks
cargo clippy -- -D warnings  # linting (treat all warnings as errors)
cargo fmt --check            # formatting check
```

## Pull request guidelines

- **Keep it offline**: `toll-optimizer` never makes network calls or collects telemetry.
- **Keep it tested**: Add unit or integration tests for any bug fixes or new features.
- **Keep it clean**: Ensure `cargo clippy -- -D warnings` and `cargo fmt` pass cleanly before submitting.
- **Keep it documented**: Update rustdoc comments and user docs if CLI behavior or options change.
- **Dependencies**: Avoid adding new external dependencies without opening an issue to discuss first.
