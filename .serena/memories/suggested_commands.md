# Suggested Commands

The following commands are used for developing and maintaining the Toll Optimizer project.

## Building and Running
- `cargo build`: Compiles the project.
- `cargo run`: Runs the application. It expects a `csv` directory containing statement files.

## Testing and Quality Control
- `cargo test`: Runs unit and integration tests.
- `cargo clippy -- -D warnings`: Runs the linter and treats all warnings as errors.
- `cargo fmt`: Formats the code according to `rustfmt` defaults.

## Project specific
- The application processes CSV files in the `csv/` directory.
- The application accepts the CSV file path as a command-line argument.
